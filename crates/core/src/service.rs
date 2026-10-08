//! Application facade used by the desktop shell. Every Tauri command maps onto one method here,
//! which keeps the shell trivial and lets the whole backend be tested without a GUI.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::catalog::{
    Benchmark, Catalog, CompanyTag, Problem, ProblemList, ProblemSummary, ReferenceSolution,
};
use crate::error::{Error, Result};
use crate::lang::{Language, Toolchains};
use crate::metrics::{self, Stats, StatsInput};
use crate::portable::{self, ImportSummary};
use crate::runner::{self, Limits, RunOptions, RunReport, TestCase};
use crate::settings::Settings;
use crate::timeutil::now_ms;
use crate::types::{Comparator, Signature};
use crate::userdb::{ProblemRef, Solve, Submission, UserDb, UserList};

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
    pub user_lists: Vec<UserList>,
    pub languages: Vec<LanguageInfo>,
    pub catalog_built_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProblemView {
    #[serde(flatten)]
    pub problem: Problem,
    pub companies: Vec<CompanyTag>,
    pub benchmarks: BTreeMap<String, Benchmark>,
    /// Highlighted reference solutions, one per available language.
    pub solutions: Vec<ReferenceSolution>,
    pub lists: Vec<String>,
    pub language: Language,
    /// Languages that can express this problem, in display order.
    pub languages: Vec<Language>,
    pub code: String,
    pub starter: String,
    pub active_solve: Option<Solve>,
    pub status: Status,
    /// The user's personal note for this problem (empty when none).
    pub note: String,
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
    /// The solve this submission counted towards; `None` for an untimed submission.
    pub solve: Option<Solve>,
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
    calibration_cancel: AtomicBool,
    /// Toolchains shipped inside the app bundle (the relocatable Python).
    bundled: BTreeMap<Language, PathBuf>,
}

/// Outcome of a local benchmark recalibration.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CalibrationSummary {
    pub measured: usize,
    pub failed: usize,
    pub cancelled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationStatus {
    pub local_benchmarks: usize,
    pub measured_at: Option<i64>,
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
    let cases: Vec<Value> = values
        .chunks(per_case)
        .map(|c| Value::Array(c.to_vec()))
        .collect();
    // Reject malformed input up front: harnesses (C++ especially) must never see it.
    for (n, case) in cases.iter().enumerate() {
        sig.check_args(case)
            .map_err(|e| Error::Invalid(format!("testcase {}: {e}", n + 1)))?;
    }
    Ok(cases)
}

/// Languages whose driver can express `sig`.
pub fn supported_languages(sig: &Signature) -> Vec<Language> {
    Language::ALL
        .iter()
        .copied()
        .filter(|l| l.driver().supports(sig))
        .collect()
}

/// Adds install guidance to "toolchain not found" errors from judging.
fn with_install_hint<T>(language: Language, r: Result<T>) -> Result<T> {
    r.map_err(|e| match e {
        Error::Toolchain(msg) => Error::Toolchain(format!(
            "{msg}. {} isn't available: {}, or set its path in Settings → Evaluation.",
            language.display_name(),
            language.install_hint()
        )),
        other => other,
    })
}

fn ensure_supported(language: Language, sig: &Signature) -> Result<()> {
    if language.driver().supports(sig) {
        Ok(())
    } else {
        Err(Error::Invalid(format!(
            "{} is not available for this problem",
            language.display_name()
        )))
    }
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

/// Resolves toolchains: explicit paths from settings win, then toolchains bundled with the app,
/// then whatever detection finds.
fn toolchains_for(settings: &Settings, bundled: &BTreeMap<Language, PathBuf>) -> Toolchains {
    let mut t = Toolchains::detect();
    for (&l, path) in bundled {
        t.set(l, path.clone());
    }
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
        let toolchains = toolchains_for(&user.settings()?, &BTreeMap::new());
        Ok(Service {
            catalog: Mutex::new(catalog),
            user: Mutex::new(user),
            summaries,
            lists,
            companies,
            toolchains: Mutex::new(toolchains),
            cache_dir: None,
            calibration_cancel: AtomicBool::new(false),
            bundled: BTreeMap::new(),
        })
    }

    /// Registers a toolchain shipped with the app; it is used unless settings name another path.
    pub fn with_bundled_toolchain(mut self, language: Language, path: PathBuf) -> Result<Service> {
        self.bundled.insert(language, path);
        *self.toolchains.lock().unwrap() = toolchains_for(&self.settings()?, &self.bundled);
        Ok(self)
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
            user_lists: self.user_lists()?,
            languages: self.languages(),
            catalog_built_at: self.catalog.lock().unwrap().meta("built_at")?,
        })
    }

    pub fn problem(&self, id: i64, language: Option<Language>) -> Result<ProblemView> {
        let (problem, companies, benchmarks, solutions) = {
            let c = self.catalog.lock().unwrap();
            (
                c.problem(id)?,
                c.companies_for(id)?,
                c.benchmarks(id)?,
                c.reference_solutions(id)?,
            )
        };
        let benchmarks = self.merge_local_benchmarks(id, benchmarks)?;
        let user = self.user.lock().unwrap();
        let languages = supported_languages(&problem.signature);
        let language = Some(language.unwrap_or(user.settings()?.language))
            .filter(|l| languages.contains(l))
            .unwrap_or(Language::Python);
        let starter = language.driver().starter_code(&problem.signature);
        let code = user
            .draft(id, language.id())?
            .map(|d| d.code)
            .unwrap_or_else(|| starter.clone());
        let active_solve = user.active_solve(&problem.summary.slug)?;
        let note = user.note(id)?.unwrap_or_default();
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
            solutions,
            lists,
            language,
            languages,
            code,
            starter,
            active_solve,
            status,
            note,
        })
    }

    pub fn save_draft(&self, id: i64, language: Language, code: &str) -> Result<()> {
        self.user
            .lock()
            .unwrap()
            .save_draft(id, language.id(), code)
    }

    /// Discards the draft and returns the starter code.
    pub fn user_lists(&self) -> Result<Vec<UserList>> {
        self.user.lock().unwrap().user_lists()
    }

    pub fn create_user_list(&self, name: &str) -> Result<Vec<UserList>> {
        let user = self.user.lock().unwrap();
        user.create_user_list(name)?;
        user.user_lists()
    }

    pub fn rename_user_list(&self, id: i64, name: &str) -> Result<Vec<UserList>> {
        let user = self.user.lock().unwrap();
        user.rename_user_list(id, name)?;
        user.user_lists()
    }

    pub fn delete_user_list(&self, id: i64) -> Result<Vec<UserList>> {
        let user = self.user.lock().unwrap();
        user.delete_user_list(id)?;
        user.user_lists()
    }

    pub fn set_in_user_list(
        &self,
        list_id: i64,
        problem_id: i64,
        member: bool,
    ) -> Result<Vec<UserList>> {
        self.summary(problem_id)?;
        let user = self.user.lock().unwrap();
        user.set_in_user_list(list_id, problem_id, member)?;
        user.user_lists()
    }

    pub fn save_note(&self, id: i64, body: &str) -> Result<()> {
        self.summary(id)?;
        self.user.lock().unwrap().save_note(id, body)
    }

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
            collect_only: false,
        })
    }

    /// Expected outputs for arbitrary inputs, produced by a bundled reference solution. The
    /// reference in `preferred` (the user's language, whose toolchain is therefore installed)
    /// is tried first, then Python.
    fn reference_outputs(
        &self,
        problem: &Problem,
        args: &[Value],
        preferred: Language,
    ) -> Result<Option<Vec<Value>>> {
        let mut candidates = vec![preferred];
        if preferred != Language::Python {
            candidates.push(Language::Python);
        }
        let cases: Vec<TestCase> = args
            .iter()
            .map(|a| TestCase {
                args: a.clone(),
                expected: Value::Null,
            })
            .collect();
        let opts = RunOptions {
            keep_all_cases: true,
            collect_only: true,
            ..self.run_options(problem, false)?
        };
        let tools = self.toolchains.lock().unwrap().clone();
        for language in candidates {
            let Some(code) = self.reference_solution(problem.summary.id, language)? else {
                continue;
            };
            let program = language.driver().program(&problem.signature, &code, &tools);
            let report = match runner::execute(
                &program,
                &cases,
                &Comparator::Exact,
                &opts,
                &mut |_, _| {},
            ) {
                Ok(r) => r,
                Err(Error::Toolchain(_)) => continue,
                Err(e) => return Err(e),
            };
            if report.cases.len() == cases.len() {
                return Ok(Some(
                    report
                        .cases
                        .into_iter()
                        .map(|c| c.output.unwrap_or(Value::Null))
                        .collect(),
                ));
            }
        }
        Ok(None)
    }

    /// "Run": executes the user's code on custom testcases, comparing with the reference solution.
    pub fn run(&self, id: i64, language: Language, code: &str, input: &str) -> Result<RunOutcome> {
        let problem = self.catalog.lock().unwrap().problem(id)?;
        ensure_supported(language, &problem.signature)?;
        let args = parse_testcases(&problem.signature, input)?;
        let tools = self.toolchains.lock().unwrap().clone();
        let program = language.driver().program(&problem.signature, code, &tools);
        let opts = RunOptions {
            collect_only: true,
            ..self.run_options(&problem, false)?
        };
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
                let reference = s.spawn(|| self.reference_outputs(&problem, &args, language));
                let user = run_user();
                (reference.join().expect("reference thread"), user)
            })
        } else {
            (
                self.reference_outputs(&problem, &args, language),
                run_user(),
            )
        };
        let mut report = with_install_hint(language, user_report)?;
        let expected = expected.ok().flatten();
        let expected_available = expected.is_some();
        // The runner only reports how the process ended (collect-only); judge the outputs here.
        // A crash, timeout or memory overrun keeps its verdict; otherwise any mismatch is a
        // wrong answer. Without expected outputs the verdict just means "finished".
        if let Some(expected) = expected {
            report.passed = 0;
            let mut first_wrong = None;
            for case in &mut report.cases {
                case.expected = expected[case.index].clone();
                case.passed = case.output.as_ref().is_some_and(|o| {
                    crate::judge::matches(&problem.comparator, &case.args, &case.expected, o)
                });
                if case.passed {
                    report.passed += 1;
                } else if case.output.is_some() && first_wrong.is_none() {
                    first_wrong = Some(case.clone());
                }
            }
            if let Some(f) = &mut report.failure {
                f.expected = expected[f.index].clone();
            }
            if report.verdict == runner::Verdict::Accepted && first_wrong.is_some() {
                report.verdict = runner::Verdict::WrongAnswer;
                report.failure = first_wrong;
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
            let b = c.benchmarks(id)?;
            (p, t, b)
        };
        let benchmark = self
            .merge_local_benchmarks(id, benchmark)?
            .remove(language.id());
        ensure_supported(language, &problem.signature)?;
        let tools = self.toolchains.lock().unwrap().clone();
        let program = language.driver().program(&problem.signature, code, &tools);
        let opts = self.run_options(&problem, true)?;
        let report = with_install_hint(
            language,
            runner::execute(&program, &tests, &problem.comparator, &opts, progress),
        )?;
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

    /// Bundled benchmarks with any measured on this machine taking precedence.
    fn merge_local_benchmarks(
        &self,
        id: i64,
        mut bundled: BTreeMap<String, Benchmark>,
    ) -> Result<BTreeMap<String, Benchmark>> {
        bundled.extend(self.user.lock().unwrap().local_benchmarks(id)?);
        Ok(bundled)
    }

    /// Re-runs every reference solution (each language with a working toolchain) `runs` times
    /// on this machine and stores the averages, so comparisons reflect local hardware. Up to
    /// `eval.max_parallel_jobs` references run concurrently. `progress(done, total)` reports
    /// after each reference; [`Service::cancel_recalibration`] stops after the current ones.
    pub fn recalibrate(
        &self,
        runs: u32,
        progress: &(dyn Fn(usize, usize) + Sync),
    ) -> Result<CalibrationSummary> {
        self.calibration_cancel.store(false, Ordering::SeqCst);
        let available: Vec<Language> = self
            .check_toolchains()
            .into_iter()
            .filter(|t| t.ok)
            .map(|t| t.language)
            .collect();
        let mut jobs = Vec::new();
        for p in &self.summaries {
            for &lang in &available {
                if let Some(code) = self.reference_solution(p.id, lang)? {
                    jobs.push((p.id, lang, code));
                }
            }
        }
        let total = jobs.len();
        let workers = self
            .settings()?
            .eval
            .max_parallel_jobs
            .clamp(1, total.max(1));
        let queue = Mutex::new(jobs.into_iter());
        let done = AtomicUsize::new(0);
        let summary = Mutex::new(CalibrationSummary::default());
        std::thread::scope(|s| {
            for _ in 0..workers {
                s.spawn(|| loop {
                    if self.calibration_cancel.load(Ordering::SeqCst) {
                        summary.lock().unwrap().cancelled = true;
                        break;
                    }
                    let Some((id, lang, code)) = queue.lock().unwrap().next() else {
                        break;
                    };
                    let ok = self.calibrate_one(id, lang, &code, runs).is_ok();
                    let mut sum = summary.lock().unwrap();
                    if ok {
                        sum.measured += 1;
                    } else {
                        sum.failed += 1;
                    }
                    drop(sum);
                    progress(done.fetch_add(1, Ordering::SeqCst) + 1, total);
                });
            }
        });
        Ok(summary.into_inner().unwrap())
    }

    fn calibrate_one(&self, id: i64, language: Language, code: &str, runs: u32) -> Result<()> {
        let (problem, tests) = {
            let c = self.catalog.lock().unwrap();
            (c.problem(id)?, c.tests(id)?)
        };
        let tools = self.toolchains.lock().unwrap().clone();
        let program = language.driver().program(&problem.signature, code, &tools);
        let opts = RunOptions {
            keep_all_cases: false,
            ..self.run_options(&problem, false)?
        };
        let (mut runtime, mut memory) = (0.0, 0u64);
        let runs = runs.max(1);
        for _ in 0..runs {
            let r = runner::execute(&program, &tests, &problem.comparator, &opts, &mut |_, _| {})?;
            if r.verdict != runner::Verdict::Accepted {
                return Err(Error::Invalid(format!(
                    "reference for problem {id} ({language}) was {:?}",
                    r.verdict
                )));
            }
            runtime += r.runtime_ms;
            memory += r.memory_kb;
        }
        let b = Benchmark {
            runtime_ms: (runtime / runs as f64 * 100.0).round() / 100.0,
            memory_kb: memory / runs as u64,
            runs,
            local: true,
        };
        self.user
            .lock()
            .unwrap()
            .save_local_benchmark(id, language.id(), &b)
    }

    pub fn cancel_recalibration(&self) {
        self.calibration_cancel.store(true, Ordering::SeqCst);
    }

    pub fn calibration_status(&self) -> Result<CalibrationStatus> {
        let (local_benchmarks, measured_at) =
            self.user.lock().unwrap().local_benchmark_summary()?;
        Ok(CalibrationStatus {
            local_benchmarks,
            measured_at,
        })
    }

    pub fn clear_local_benchmarks(&self) -> Result<()> {
        self.user.lock().unwrap().clear_local_benchmarks()
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
        *self.toolchains.lock().unwrap() = toolchains_for(&saved, &self.bundled);
        Ok(saved)
    }

    pub fn stats(
        &self,
        recent_n: usize,
        review_min_days: f64,
        utc_offset_minutes: i32,
    ) -> Result<Stats> {
        let (submissions, solves, user_lists) = {
            let u = self.user.lock().unwrap();
            (u.activity()?, u.completed_solves()?, u.user_lists()?)
        };
        // Progress covers the user's own lists after the bundled ones.
        let lists: Vec<ProblemList> = self
            .lists
            .iter()
            .cloned()
            .chain(user_lists.into_iter().map(|l| {
                ProblemList {
                    id: format!("user:{}", l.id),
                    name: l.name,
                    description: String::new(),
                    items: l
                        .problem_ids
                        .into_iter()
                        .map(|problem_id| crate::catalog::ListItem {
                            problem_id,
                            group: String::new(),
                        })
                        .collect(),
                }
            }))
            .collect();
        Ok(metrics::compute(&StatsInput {
            problems: &self.summaries,
            lists: &lists,
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

    /// Writes the portable history file (see [`portable`]) to `path`.
    pub fn export_to(&self, path: &Path) -> Result<()> {
        let data = portable::export(&self.user.lock().unwrap(), &self.summaries)?;
        std::fs::write(path, serde_json::to_string_pretty(&data)?)?;
        Ok(())
    }

    /// Merges a portable history file into the user database.
    pub fn import_from(&self, path: &Path, apply_settings: bool) -> Result<ImportSummary> {
        let json = std::fs::read_to_string(path)?;
        let summary = portable::import(&self.user.lock().unwrap(), &json, apply_settings)?;
        if summary.settings_applied {
            *self.toolchains.lock().unwrap() = toolchains_for(&self.settings()?, &self.bundled);
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
        assert_eq!(v.solutions.len(), 2);
        assert!(svc.problem(42, None).is_err());

        svc.save_note(1, "hash map").unwrap();
        assert_eq!(svc.problem(1, None).unwrap().note, "hash map");
        assert!(svc.save_note(404, "x").is_err());
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
        assert_eq!(wa.solve.unwrap().errors, 1);
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
        let solve = ac.solve.unwrap();
        assert_eq!(solve.completed_at, Some(ac.submission.created_at));
        assert_eq!(solve.elapsed_ms, 3000);
        assert_eq!(ac.benchmark.unwrap().runs, 10);
        assert_eq!(svc.submissions(1).unwrap().len(), 2);
        assert_eq!(svc.bootstrap().unwrap().problems[0].status, Status::Solved);
        assert_eq!(svc.problem(1, None).unwrap().status, Status::Solved);
        assert_eq!(svc.solves().unwrap().len(), 1);

        let stats = svc.stats(5, 0.0, 0).unwrap();
        assert_eq!(stats.totals.solved, 1);
        assert_eq!(stats.totals.submissions, 2);

        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("history.json");
        svc.export_to(&file).unwrap();
        svc.clear_history().unwrap();
        assert!(svc.submissions(1).unwrap().is_empty());
        let summary = svc.import_from(&file, true).unwrap();
        assert_eq!(summary.submissions_added, 2);
        assert_eq!(svc.submissions(1).unwrap().len(), 2);
        assert!(svc
            .import_from(&dir.path().join("missing.json"), false)
            .is_err());
        assert!(svc.export_to(&dir.path().join("no/such/dir.json")).is_err());
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
        let err = svc
            .submit(1, Language::Python, GOOD, 0, &mut |_, _| {})
            .unwrap_err();
        assert!(matches!(err, Error::Toolchain(_)));
        assert!(err.to_string().contains("Install Python 3"), "{err}");
        let err = svc.run(1, Language::Python, GOOD, "[1,2]\n3").unwrap_err();
        assert!(err.to_string().contains("Settings → Evaluation"), "{err}");
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
    fn run_reports_crashes_and_timeouts_instead_of_wrong_answers() {
        let svc = service();
        let mut s = svc.settings().unwrap();
        s.eval.time_limit_multiplier = 0.25; // 500 ms budget for the sample problem
        svc.save_settings(&s).unwrap();
        let input = "[2,7,11,15]\n9\n[3,2,4]\n6";

        let hang = "class Solution:\n    def twoSum(self, nums, target):\n        if target == 6:\n            while True: pass\n        return [0, 1]\n";
        let start = std::time::Instant::now();
        let r = svc.run(1, Language::Python, hang, input).unwrap();
        assert!(start.elapsed() < std::time::Duration::from_secs(8));
        assert_eq!(r.report.verdict, Verdict::TimeLimitExceeded);
        assert_eq!(r.report.cases.len(), 2);
        assert!(r.report.cases[0].passed);
        let f = r.report.failure.unwrap();
        assert_eq!(
            (f.index, f.expected),
            (1, json!([1, 2])),
            "the hung case shows its expected output"
        );

        let crash = "class Solution:\n    def twoSum(self, nums, target):\n        print('debug', target)\n        if target == 6:\n            raise ValueError('boom')\n        return [0, 1]\n";
        let r = svc.run(1, Language::Python, crash, input).unwrap();
        assert_eq!(r.report.verdict, Verdict::RuntimeError);
        assert!(r.report.error.unwrap().contains("ValueError: boom"));
        assert_eq!(
            r.report.cases[1].stdout, "debug 6\n",
            "the crashing case keeps its output"
        );

        let err = svc
            .run(1, Language::Python, GOOD, "[1,2]\n\"x\"")
            .unwrap_err();
        assert!(
            err.to_string()
                .contains("testcase 1: `target` is not a valid int"),
            "{err}"
        );
    }

    #[test]
    fn run_prefers_a_same_language_reference() {
        let svc = service();
        let js = "var twoSum = function(nums, target) { for (let i = 0; i < nums.length; i++) for (let j = i + 1; j < nums.length; j++) if (nums[i] + nums[j] === target) return [i, j]; };";
        let r = svc
            .run(1, Language::JavaScript, js, "[2,7,11,15]\n9")
            .unwrap();
        assert_eq!(
            r.report.cases[0].expected,
            json!([1, 0]),
            "JavaScript reference used"
        );
        assert_eq!(
            r.report.verdict,
            Verdict::Accepted,
            "unordered comparator accepts either order"
        );
        let py = svc
            .run(1, Language::Python, GOOD, "[2,7,11,15]\n9")
            .unwrap();
        assert_eq!(
            py.report.cases[0].expected,
            json!([0, 1]),
            "Python reference used"
        );

        // No C++ reference: falls back to Python.
        let problem = sample_catalog().problem(1).unwrap();
        let args = vec![json!([[3, 2, 4], 6])];
        let out = svc
            .reference_outputs(&problem, &args, Language::Cpp)
            .unwrap()
            .unwrap();
        assert_eq!(out, vec![json!([1, 2])]);

        // A missing toolchain for the preferred reference also falls back.
        let mut s = svc.settings().unwrap();
        s.eval
            .toolchain_paths
            .insert("javascript".into(), "/nonexistent/node".into());
        svc.save_settings(&s).unwrap();
        let out = svc
            .reference_outputs(&problem, &args, Language::JavaScript)
            .unwrap()
            .unwrap();
        assert_eq!(out, vec![json!([1, 2])]);
    }

    #[test]
    fn language_support_per_problem() {
        let graph: Signature = serde_json::from_value(json!({"kind": "function", "name": "cloneGraph",
            "special": "deep_copy", "params": [{"name": "node", "type": "Node"}], "returns": "Node"}))
        .unwrap();
        let langs = supported_languages(&graph);
        assert!(langs.contains(&Language::Python) && !langs.contains(&Language::Rust));
        assert!(ensure_supported(Language::Rust, &graph)
            .unwrap_err()
            .to_string()
            .contains("Rust is not available"));

        let svc = service();
        let mut s = svc.settings().unwrap();
        s.language = Language::Rust;
        svc.save_settings(&s).unwrap();
        let v = svc.problem(1, None).unwrap();
        assert_eq!(v.language, Language::Rust);
        assert!(v.languages.contains(&Language::Cpp));
        assert!(v.starter.contains("pub fn two_sum"));
    }

    #[test]
    fn local_recalibration() {
        let svc = service();
        assert_eq!(svc.calibration_status().unwrap().local_benchmarks, 0);
        let seen = Mutex::new(Vec::new());
        let summary = svc
            .recalibrate(1, &|d, t| seen.lock().unwrap().push((d, t)))
            .unwrap();
        // Problem 1 has Python and JavaScript references; the others have none.
        assert_eq!(
            summary,
            CalibrationSummary {
                measured: 2,
                failed: 0,
                cancelled: false
            }
        );
        assert_eq!(seen.lock().unwrap().last(), Some(&(2, 2)));
        let status = svc.calibration_status().unwrap();
        assert_eq!(status.local_benchmarks, 2);
        assert!(status.measured_at.is_some());
        let view = svc.problem(1, None).unwrap();
        assert!(
            view.benchmarks["python"].local,
            "local numbers replace bundled ones"
        );
        assert_eq!(view.benchmarks["python"].runs, 1);
        let ac = svc
            .submit(1, Language::Python, GOOD, 0, &mut |_, _| {})
            .unwrap();
        assert!(ac.benchmark.unwrap().local);

        svc.cancel_recalibration();
        // A new run resets the flag; cancelling from inside the progress callback stops early.
        let s = svc
            .recalibrate(1, &|_, _| svc.cancel_recalibration())
            .unwrap();
        assert!(s.cancelled || s.measured <= 2);

        svc.clear_local_benchmarks().unwrap();
        assert!(!svc.problem(1, None).unwrap().benchmarks["python"].local);
    }

    #[test]
    fn calibration_counts_failures() {
        let w = crate::catalog::CatalogWriter::in_memory().unwrap();
        let p = crate::catalog::tests::sample_problem(1, "two-sum", crate::types::Difficulty::Easy);
        w.add_problem(
            &p,
            &[TestCase {
                args: json!([[2, 7], 9]),
                expected: json!([0, 1]),
            }],
        )
        .unwrap();
        w.add_solution(
            1,
            Language::Python,
            "class Solution:\n    def twoSum(self, nums, target):\n        return [9, 9]\n",
            "",
        )
        .unwrap();
        let svc = Service::new(w.finish().unwrap(), UserDb::in_memory().unwrap()).unwrap();
        let s = svc.recalibrate(1, &|_, _| {}).unwrap();
        assert_eq!((s.measured, s.failed), (0, 1));
    }

    #[test]
    fn bundled_toolchains_sit_between_settings_and_detection() {
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("python3");
        std::fs::write(&fake, "").unwrap();
        let svc = service()
            .with_bundled_toolchain(Language::Python, fake.clone())
            .unwrap();
        assert_eq!(svc.languages()[0].executable, fake.to_string_lossy());
        let mut s = svc.settings().unwrap();
        s.eval
            .toolchain_paths
            .insert("python".into(), "/custom/python3".into());
        svc.save_settings(&s).unwrap();
        assert_eq!(svc.languages()[0].executable, "/custom/python3");
        s.eval.toolchain_paths.clear();
        svc.save_settings(&s).unwrap();
        assert_eq!(svc.languages()[0].executable, fake.to_string_lossy());
    }

    #[test]
    fn user_lists_through_the_service() {
        let svc = service();
        let lists = svc.create_user_list("Favorites").unwrap();
        let id = lists[0].id;
        assert_eq!(
            svc.set_in_user_list(id, 1, true).unwrap()[0].problem_ids,
            vec![1]
        );
        assert!(svc.set_in_user_list(id, 404, true).is_err());
        assert_eq!(svc.bootstrap().unwrap().user_lists.len(), 1);
        svc.submit(1, Language::Python, GOOD, 0, &mut |_, _| {})
            .unwrap();
        let stats = svc.stats(5, 0.0, 0).unwrap();
        let fav = stats.lists.iter().find(|l| l.name == "Favorites").unwrap();
        assert_eq!((fav.solved, fav.total), (1, 1));
        assert_eq!(svc.rename_user_list(id, "Saved").unwrap()[0].name, "Saved");
        assert!(svc.delete_user_list(id).unwrap().is_empty());
    }

    #[test]
    fn opens_from_files() {
        let dir = tempfile::tempdir().unwrap();
        assert!(Service::open(&dir.path().join("missing.db"), &dir.path().join("u.db")).is_err());
    }
}
