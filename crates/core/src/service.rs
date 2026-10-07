//! Application facade used by the desktop shell. Every Tauri command maps onto one method here,
//! which keeps the shell trivial and lets the whole backend be tested without a GUI.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::catalog::{Benchmark, Catalog, CompanyTag, Problem, ProblemList, ProblemSummary};
use crate::error::{Error, Result};
use crate::lang::{Language, Toolchains};
use crate::metrics::{self, Stats, StatsInput};
use crate::portable::{self, ImportSummary};
use crate::runner::{self, Limits, RunOptions, RunReport, TestCase};
use crate::settings::Settings;
use crate::timeutil::now_ms;
use crate::types::{Comparator, Signature};
use crate::userdb::{ProblemRef, Solve, Submission, UserDb};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Solved,
    Attempted,
    Todo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProblemListing {
    #[serde(flatten)]
    pub summary: ProblemSummary,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanyEntry {
    pub name: String,
    pub problem_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub id: Language,
    pub name: String,
    pub available: bool,
    pub executable: String,
}

/// Everything the UI needs for its first paint, in one round trip.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bootstrap {
    pub problems: Vec<ProblemListing>,
    pub lists: Vec<ProblemList>,
    pub companies: Vec<CompanyEntry>,
    pub topics: Vec<String>,
    pub settings: Settings,
    pub languages: Vec<LanguageInfo>,
    pub catalog_built_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProblemView {
    #[serde(flatten)]
    pub problem: Problem,
    pub companies: Vec<CompanyTag>,
    pub benchmarks: BTreeMap<String, Benchmark>,
    pub lists: Vec<String>,
    pub language: Language,
    pub code: String,
    pub starter: String,
    pub active_solve: Option<Solve>,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunOutcome {
    #[serde(flatten)]
    pub report: RunReport,
    /// False when no reference solution could produce expected outputs for custom input.
    pub expected_available: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitOutcome {
    pub submission: Submission,
    pub solve: Solve,
    pub benchmark: Option<Benchmark>,
}

pub struct Service {
    catalog: Mutex<Catalog>,
    user: Mutex<UserDb>,
    summaries: Vec<ProblemSummary>,
    lists: Vec<ProblemList>,
    companies: Vec<CompanyEntry>,
    toolchains: Mutex<Toolchains>,
    cache_dir: Option<PathBuf>,
}

/// Parses LeetCode-style testcase text (one JSON value per line, one group of lines per case).
pub fn parse_testcases(sig: &Signature, text: &str) -> Result<Vec<Value>> {
    let per_case = sig.arity();
    let values: Vec<Value> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, l)| {
            serde_json::from_str(l)
                .map_err(|e| Error::Invalid(format!("testcase line {}: {e}", i + 1)))
        })
        .collect::<Result<_>>()?;
    if per_case == 0 {
        return Ok(vec![Value::Array(vec![])]);
    }
    if values.is_empty() || !values.len().is_multiple_of(per_case) {
        return Err(Error::Invalid(format!(
            "each testcase needs {per_case} line(s); got {} line(s)",
            values.len()
        )));
    }
    Ok(values
        .chunks(per_case)
        .map(|c| Value::Array(c.to_vec()))
        .collect())
}

fn limits_for(problem: &Problem, settings: &Settings) -> Limits {
    let time = (problem.time_limit_ms as f64 * settings.eval.time_limit_multiplier) as u64;
    Limits {
        time_limit_ms: time,
        wall_limit_ms: time * 3 + 10_000,
        memory_limit_kb: settings.eval.memory_limit_mb * 1024,
        ..Limits::default()
    }
}

fn toolchains_for(settings: &Settings) -> Toolchains {
    let mut t = Toolchains::detect();
    for &l in Language::ALL {
        if let Some(p) = settings.eval.toolchain_paths.get(l.id()) {
            if !p.trim().is_empty() {
                t.set(l, PathBuf::from(p.trim()));
            }
        }
    }
    t
}

impl Service {
    pub fn new(catalog: Catalog, user: UserDb) -> Result<Service> {
        let summaries = catalog.summaries()?;
        let lists = catalog.lists()?;
        let mut companies: Vec<CompanyEntry> = catalog
            .company_index()?
            .into_iter()
            .map(|(name, problem_ids)| CompanyEntry { name, problem_ids })
            .collect();
        companies.sort_by(|a, b| {
            b.problem_ids
                .len()
                .cmp(&a.problem_ids.len())
                .then(a.name.cmp(&b.name))
        });
        let toolchains = toolchains_for(&user.settings()?);
        Ok(Service {
            catalog: Mutex::new(catalog),
            user: Mutex::new(user),
            summaries,
            lists,
            companies,
            toolchains: Mutex::new(toolchains),
            cache_dir: None,
        })
    }

    /// Caches compiled programs under `dir` so Run followed by Submit compiles once.
    pub fn with_cache_dir(mut self, dir: PathBuf) -> Service {
        self.cache_dir = Some(dir);
        self
    }

    pub fn open(catalog_path: &Path, user_path: &Path) -> Result<Service> {
        Service::new(Catalog::open(catalog_path)?, UserDb::open(user_path)?)
    }

    fn statuses(&self) -> Result<HashMap<i64, Status>> {
        Ok(self
            .user
            .lock()
            .unwrap()
            .problem_statuses()?
            .into_iter()
            .map(|(id, ok)| {
                (
                    id,
                    if ok {
                        Status::Solved
                    } else {
                        Status::Attempted
                    },
                )
            })
            .collect())
    }

    fn summary(&self, id: i64) -> Result<&ProblemSummary> {
        self.summaries
            .iter()
            .find(|p| p.id == id)
            .ok_or_else(|| Error::NotFound(format!("problem {id}")))
    }

    fn problem_ref(&self, id: i64) -> Result<ProblemRef> {
        let s = self.summary(id)?;
        Ok(ProblemRef {
            id: Some(s.id),
            slug: s.slug.clone(),
            title: s.title.clone(),
            difficulty: s.difficulty,
        })
    }

    pub fn languages(&self) -> Vec<LanguageInfo> {
        let tools = self.toolchains.lock().unwrap().clone();
        Language::ALL
            .iter()
            .map(|&l| {
                let exe = tools.path(l);
                LanguageInfo {
                    id: l,
                    name: l.display_name().into(),
                    available: exe.is_absolute() && exe.is_file(),
                    executable: exe.to_string_lossy().into_owned(),
                }
            })
            .collect()
    }

    /// Probes every language toolchain (spawns processes; call off the UI thread).
    pub fn check_toolchains(&self) -> Vec<crate::lang::ToolchainStatus> {
        let tools = self.toolchains.lock().unwrap().clone();
        Language::ALL
            .iter()
            .map(|&l| crate::lang::probe(l, &tools))
            .collect()
    }

    pub fn bootstrap(&self) -> Result<Bootstrap> {
        let statuses = self.statuses()?;
        let topics: BTreeSet<String> = self
            .summaries
            .iter()
            .flat_map(|p| p.topics.clone())
            .collect();
        Ok(Bootstrap {
            problems: self
                .summaries
                .iter()
                .map(|s| ProblemListing {
                    summary: s.clone(),
                    status: statuses.get(&s.id).copied().unwrap_or(Status::Todo),
                })
                .collect(),
            lists: self.lists.clone(),
            companies: self.companies.clone(),
            topics: topics.into_iter().collect(),
            settings: self.settings()?,
            languages: self.languages(),
            catalog_built_at: self.catalog.lock().unwrap().meta("built_at")?,
        })
    }

    pub fn problem(&self, id: i64, language: Option<Language>) -> Result<ProblemView> {
        let (problem, companies, benchmarks) = {
            let c = self.catalog.lock().unwrap();
            (c.problem(id)?, c.companies_for(id)?, c.benchmarks(id)?)
        };
        let user = self.user.lock().unwrap();
        let language = language.unwrap_or(user.settings()?.language);
        let starter = language.driver().starter_code(&problem.signature);
        let code = user
            .draft(id, language.id())?
            .map(|d| d.code)
            .unwrap_or_else(|| starter.clone());
        let active_solve = user.active_solve(&problem.summary.slug)?;
        let status = user
            .problem_statuses()?
            .into_iter()
            .find(|(pid, _)| *pid == id)
            .map(|(_, ok)| {
                if ok {
                    Status::Solved
                } else {
                    Status::Attempted
                }
            })
            .unwrap_or(Status::Todo);
        let lists = self
            .lists
            .iter()
            .filter(|l| l.items.iter().any(|i| i.problem_id == id))
            .map(|l| l.name.clone())
            .collect();
        Ok(ProblemView {
            problem,
            companies,
            benchmarks,
            lists,
            language,
            code,
            starter,
            active_solve,
            status,
        })
    }

    pub fn save_draft(&self, id: i64, language: Language, code: &str) -> Result<()> {
        self.user
            .lock()
            .unwrap()
            .save_draft(id, language.id(), code)
    }

    /// Discards the draft and returns the starter code.
    pub fn reset_code(&self, id: i64, language: Language) -> Result<String> {
        let sig = self.catalog.lock().unwrap().problem(id)?.signature;
        self.user.lock().unwrap().delete_draft(id, language.id())?;
        Ok(language.driver().starter_code(&sig))
    }

    pub fn reference_solution(&self, id: i64, language: Language) -> Result<Option<String>> {
        self.catalog.lock().unwrap().solution(id, language)
    }

    fn run_options(&self, problem: &Problem, stop_on_failure: bool) -> Result<RunOptions> {
        let settings = self.settings()?;
        Ok(RunOptions {
            limits: limits_for(problem, &settings),
            stop_on_failure,
            keep_all_cases: !stop_on_failure,
            low_priority: settings.eval.low_priority,
            cache_dir: self.cache_dir.clone(),
        })
    }

    /// Expected outputs for arbitrary inputs, produced by the bundled reference solution.
    fn reference_outputs(&self, problem: &Problem, args: &[Value]) -> Result<Option<Vec<Value>>> {
        let Some(code) = self.reference_solution(problem.summary.id, Language::Python)? else {
            return Ok(None);
        };
        let tools = self.toolchains.lock().unwrap().clone();
        let program = Language::Python
            .driver()
            .program(&problem.signature, &code, &tools);
        let cases: Vec<TestCase> = args
            .iter()
            .map(|a| TestCase {
                args: a.clone(),
                expected: Value::Null,
            })
            .collect();
        let opts = RunOptions {
            keep_all_cases: true,
            ..self.run_options(problem, false)?
        };
        let report = runner::execute(&program, &cases, &Comparator::Exact, &opts, &mut |_, _| {})?;
        if report.cases.len() != cases.len() {
            return Ok(None);
        }
        Ok(Some(
            report
                .cases
                .into_iter()
                .map(|c| c.output.unwrap_or(Value::Null))
                .collect(),
        ))
    }

    /// "Run": executes the user's code on custom testcases, comparing with the reference solution.
    pub fn run(&self, id: i64, language: Language, code: &str, input: &str) -> Result<RunOutcome> {
        let problem = self.catalog.lock().unwrap().problem(id)?;
        let args = parse_testcases(&problem.signature, input)?;
        let tools = self.toolchains.lock().unwrap().clone();
        let program = language.driver().program(&problem.signature, code, &tools);
        let opts = self.run_options(&problem, false)?;
        let cases: Vec<TestCase> = args
            .iter()
            .map(|a| TestCase {
                args: a.clone(),
                expected: Value::Null,
            })
            .collect();
        let run_user =
            || runner::execute(&program, &cases, &Comparator::Exact, &opts, &mut |_, _| {});
        let (expected, user_report) = if self.settings()?.eval.max_parallel_jobs > 1 {
            std::thread::scope(|s| {
                let reference = s.spawn(|| self.reference_outputs(&problem, &args));
                let user = run_user();
                (reference.join().expect("reference thread"), user)
            })
        } else {
            (self.reference_outputs(&problem, &args), run_user())
        };
        let mut report = user_report?;
        let expected = expected.ok().flatten();
        let expected_available = expected.is_some();
        if let Some(expected) = expected {
            report.passed = 0;
            report.failure = None;
            for case in &mut report.cases {
                case.expected = expected[case.index].clone();
                case.passed = case.output.as_ref().is_some_and(|o| {
                    crate::judge::matches(&problem.comparator, &case.args, &case.expected, o)
                });
                if case.passed {
                    report.passed += 1;
                } else if report.failure.is_none() {
                    report.failure = Some(case.clone());
                }
            }
            if report.verdict == runner::Verdict::WrongAnswer && report.passed == report.total {
                report.verdict = runner::Verdict::Accepted;
            } else if report.verdict == runner::Verdict::Accepted && report.passed < report.total {
                report.verdict = runner::Verdict::WrongAnswer;
            }
        }
        Ok(RunOutcome {
            report,
            expected_available,
        })
    }

    /// "Submit": judges against the full suite, records the submission and updates the solve.
    pub fn submit(
        &self,
        id: i64,
        language: Language,
        code: &str,
        elapsed_ms: i64,
        progress: &mut dyn FnMut(usize, usize),
    ) -> Result<SubmitOutcome> {
        let (problem, tests, benchmark) = {
            let c = self.catalog.lock().unwrap();
            let p = c.problem(id)?;
            let t = c.tests(id)?;
            let b = c.benchmarks(id)?.remove(language.id());
            (p, t, b)
        };
        let tools = self.toolchains.lock().unwrap().clone();
        let program = language.driver().program(&problem.signature, code, &tools);
        let opts = self.run_options(&problem, true)?;
        let report = runner::execute(&program, &tests, &problem.comparator, &opts, progress)?;
        let pref = self.problem_ref(id)?;
        let user = self.user.lock().unwrap();
        let (submission, solve) = user.transaction(|db| {
            let submission = db.insert_submission(id, language.id(), code, &report, now_ms())?;
            let solve = db.record_attempt(&pref, &submission, elapsed_ms)?;
            Ok((submission, solve))
        })?;
        Ok(SubmitOutcome {
            submission,
            solve,
            benchmark,
        })
    }

    pub fn submissions(&self, id: i64) -> Result<Vec<Submission>> {
        self.user.lock().unwrap().submissions_for(id)
    }

    pub fn start_solve(&self, id: i64) -> Result<Solve> {
        let pref = self.problem_ref(id)?;
        self.user.lock().unwrap().start_solve(&pref, now_ms())
    }

    pub fn update_solve(&self, solve_id: i64, elapsed_ms: i64) -> Result<()> {
        self.user
            .lock()
            .unwrap()
            .set_solve_elapsed(solve_id, elapsed_ms)
    }

    pub fn discard_solve(&self, solve_id: i64) -> Result<()> {
        self.user.lock().unwrap().discard_solve(solve_id)
    }

    pub fn settings(&self) -> Result<Settings> {
        self.user.lock().unwrap().settings()
    }

    pub fn save_settings(&self, settings: &Settings) -> Result<Settings> {
        let saved = self.user.lock().unwrap().save_settings(settings)?;
        *self.toolchains.lock().unwrap() = toolchains_for(&saved);
        Ok(saved)
    }

    pub fn stats(
        &self,
        recent_n: usize,
        review_min_days: f64,
        utc_offset_minutes: i32,
    ) -> Result<Stats> {
        let (submissions, solves) = {
            let u = self.user.lock().unwrap();
            (u.all_submissions()?, u.completed_solves()?)
        };
        Ok(metrics::compute(&StatsInput {
            problems: &self.summaries,
            lists: &self.lists,
            submissions: &submissions,
            solves: &solves,
            now_ms: now_ms(),
            recent_n: recent_n.max(1),
            review_min_days: review_min_days.max(0.0),
            utc_offset_minutes,
        }))
    }

    pub fn solves(&self) -> Result<Vec<Solve>> {
        self.user.lock().unwrap().completed_solves()
    }

    pub fn export_json(&self) -> Result<String> {
        let data = portable::export(&self.user.lock().unwrap(), &self.summaries)?;
        Ok(serde_json::to_string_pretty(&data)?)
    }

    pub fn import_json(&self, json: &str, apply_settings: bool) -> Result<ImportSummary> {
        let summary = portable::import(&self.user.lock().unwrap(), json, apply_settings)?;
        if summary.settings_applied {
            *self.toolchains.lock().unwrap() = toolchains_for(&self.settings()?);
        }
        Ok(summary)
    }

    pub fn clear_history(&self) -> Result<()> {
        self.user.lock().unwrap().clear_history()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::tests::sample_catalog;
    use crate::runner::Verdict;
    use serde_json::json;

    fn service() -> Service {
        Service::new(sample_catalog(), UserDb::in_memory().unwrap()).unwrap()
    }

    const GOOD: &str = "class Solution:\n    def twoSum(self, nums, target):\n        d = {}\n        for i, n in enumerate(nums):\n            if target - n in d: return [d[target - n], i]\n            d[n] = i\n";
    const BAD: &str =
        "class Solution:\n    def twoSum(self, nums, target):\n        return [0, 0]\n";

    #[test]
    fn testcase_parsing() {
        let sig: Signature = serde_json::from_value(json!({"kind": "function", "name": "f",
            "params": [{"name": "a", "type": "int[]"}, {"name": "b", "type": "int"}], "returns": "int"}))
        .unwrap();
        let cases = parse_testcases(&sig, "[1,2]\n3\n\n[4]\n5\n").unwrap();
        assert_eq!(cases, vec![json!([[1, 2], 3]), json!([[4], 5])]);
        assert!(parse_testcases(&sig, "[1,2]\n").is_err());
        assert!(parse_testcases(&sig, "").is_err());
        let err = parse_testcases(&sig, "[1,\n2").unwrap_err().to_string();
        assert!(err.contains("line 1"), "{err}");
        let design: Signature = serde_json::from_value(json!({"kind": "design", "class": "C",
            "methods": [{"name": "m", "returns": "int"}]}))
        .unwrap();
        assert_eq!(
            parse_testcases(&design, "[\"C\",\"m\"]\n[[],[]]")
                .unwrap()
                .len(),
            1
        );
        let nullary: Signature = serde_json::from_value(json!({"kind": "function", "name": "f",
            "params": [], "returns": "int"}))
        .unwrap();
        assert_eq!(parse_testcases(&nullary, "").unwrap(), vec![json!([])]);
    }

    #[test]
    fn bootstrap_and_problem_view() {
        let svc = service();
        let b = svc.bootstrap().unwrap();
        assert_eq!(b.problems.len(), 3);
        assert_eq!(b.problems[0].status, Status::Todo);
        assert_eq!(b.companies[0].name, "Google");
        assert_eq!(b.topics, vec!["Array".to_string()]);
        assert_eq!(b.catalog_built_at.as_deref(), Some("2026-10-07"));
        assert_eq!(b.languages.len(), Language::ALL.len());

        let v = svc.problem(1, None).unwrap();
        assert_eq!(v.code, v.starter);
        assert!(v.starter.contains("def twoSum"));
        assert_eq!(v.lists, vec!["Blind 75".to_string()]);
        assert_eq!(v.companies.len(), 2);
        assert!(v.benchmarks.contains_key("python"));
        assert!(svc.problem(42, None).is_err());

        svc.save_draft(1, Language::Python, "mine").unwrap();
        assert_eq!(svc.problem(1, Some(Language::Python)).unwrap().code, "mine");
        let starter = svc.reset_code(1, Language::Python).unwrap();
        assert_eq!(svc.problem(1, None).unwrap().code, starter);
    }

    #[test]
    fn run_compares_against_reference() {
        let svc = service();
        let ok = svc
            .run(1, Language::Python, GOOD, "[2,7,11,15]\n9\n[3,2,4]\n6")
            .unwrap();
        assert!(ok.expected_available);
        assert_eq!(ok.report.verdict, Verdict::Accepted, "{:?}", ok.report);
        assert_eq!(ok.report.cases.len(), 2);
        assert_eq!(ok.report.cases[1].expected, json!([1, 2]));

        let bad = svc.run(1, Language::Python, BAD, "[2,7,11,15]\n9").unwrap();
        assert_eq!(bad.report.verdict, Verdict::WrongAnswer);
        assert_eq!(bad.report.failure.unwrap().output, Some(json!([0, 0])));

        // No reference solution for problem 2: outputs are shown without expectations.
        let none = svc.run(2, Language::Python, GOOD, "[1,2]\n3").unwrap();
        assert!(!none.expected_available);
        assert_eq!(none.report.cases[0].output, Some(json!([0, 1])));

        assert!(svc.run(1, Language::Python, GOOD, "[1]").is_err());

        // One judge process at a time, at low priority, gives the same answers.
        let mut s = svc.settings().unwrap();
        s.eval.max_parallel_jobs = 1;
        s.eval.low_priority = true;
        svc.save_settings(&s).unwrap();
        let seq = svc.run(1, Language::Python, GOOD, "[3,2,4]\n6").unwrap();
        assert_eq!(seq.report.verdict, Verdict::Accepted);
    }

    #[test]
    fn submit_records_history_and_solves() {
        let svc = service();
        let solve = svc.start_solve(1).unwrap();
        svc.update_solve(solve.id, 1234).unwrap();
        let mut progress = vec![];
        let wa = svc
            .submit(1, Language::Python, BAD, 2000, &mut |d, t| {
                progress.push((d, t))
            })
            .unwrap();
        assert_eq!(wa.submission.verdict, Verdict::WrongAnswer);
        assert_eq!(wa.solve.errors, 1);
        assert!(!progress.is_empty());
        let ac = svc
            .submit(1, Language::Python, GOOD, 3000, &mut |_, _| {})
            .unwrap();
        assert_eq!(
            ac.submission.verdict,
            Verdict::Accepted,
            "{:?}",
            ac.submission
        );
        assert_eq!(ac.solve.completed_at, Some(ac.submission.created_at));
        assert_eq!(ac.solve.elapsed_ms, 3000);
        assert_eq!(ac.benchmark.unwrap().runs, 10);
        assert_eq!(svc.submissions(1).unwrap().len(), 2);
        assert_eq!(svc.bootstrap().unwrap().problems[0].status, Status::Solved);
        assert_eq!(svc.problem(1, None).unwrap().status, Status::Solved);
        assert_eq!(svc.solves().unwrap().len(), 1);

        let stats = svc.stats(5, 0.0, 0).unwrap();
        assert_eq!(stats.totals.solved, 1);
        assert_eq!(stats.totals.submissions, 2);

        let exported = svc.export_json().unwrap();
        svc.clear_history().unwrap();
        assert!(svc.submissions(1).unwrap().is_empty());
        let summary = svc.import_json(&exported, true).unwrap();
        assert_eq!(summary.submissions_added, 2);
        assert_eq!(svc.submissions(1).unwrap().len(), 2);
    }

    #[test]
    fn solve_discard_and_unknown_problem() {
        let svc = service();
        let s = svc.start_solve(1).unwrap();
        svc.discard_solve(s.id).unwrap();
        assert!(svc.problem(1, None).unwrap().active_solve.is_none());
        assert!(svc.start_solve(77).is_err());
    }

    #[test]
    fn settings_update_toolchains() {
        let svc = service();
        let mut s = svc.settings().unwrap();
        s.eval
            .toolchain_paths
            .insert("python".into(), "/nonexistent/python3".into());
        s.eval.toolchain_paths.insert("cobol".into(), "/x".into());
        s.eval.time_limit_multiplier = 2.0;
        let saved = svc.save_settings(&s).unwrap();
        assert_eq!(saved.eval.time_limit_multiplier, 2.0);
        let langs = svc.languages();
        assert!(!langs[0].available);
        assert_eq!(langs[0].executable, "/nonexistent/python3");
        assert!(matches!(
            svc.submit(1, Language::Python, GOOD, 0, &mut |_, _| {}),
            Err(Error::Toolchain(_))
        ));
        let problem = sample_catalog().problem(1).unwrap();
        let limits = limits_for(&problem, &saved);
        assert_eq!(limits.time_limit_ms, 4000);
        assert_eq!(limits.memory_limit_kb, 1024 * 1024);
    }

    #[test]
    fn toolchain_checks() {
        let dir = tempfile::tempdir().unwrap();
        let svc = service().with_cache_dir(dir.path().join("compiled"));
        let ok = svc.run(1, Language::Python, GOOD, "[3,2,4]\n6").unwrap();
        assert_eq!(ok.report.verdict, Verdict::Accepted);
        assert!(svc.check_toolchains().iter().all(|t| t.ok));
    }

    #[test]
    fn opens_from_files() {
        let dir = tempfile::tempdir().unwrap();
        assert!(Service::open(&dir.path().join("missing.db"), &dir.path().join("u.db")).is_err());
    }
}
