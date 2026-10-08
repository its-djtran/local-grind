//! Builds the static problem catalog (`problems.db`) from the `content/` tree.
//!
//! Layout of one problem directory (`content/problems/<slug>/`):
//! * `problem.toml`   — metadata, signature, judge mode, examples, hints
//! * `description.md` — statement; `<!-- examples -->` marks where examples are rendered
//! * `editorial.md`   — explanation of the optimal approach
//! * `solution.py`    — reference solution; produces every expected output
//! * `gen.py`         — prints one JSON argument array per line (generated tests)
//! * `benchmark.json` — optional; reference runtime/memory written by `lg-content bench`

use localgrind_core::catalog::{
    Benchmark, CatalogWriter, CompanyTag, Example, ListItem, Problem, ProblemList, ProblemSummary,
};
use localgrind_core::judge;
use localgrind_core::lang::{Language, Toolchains};
use localgrind_core::runner::{self, Limits, RunOptions, TestCase, Verdict};
use localgrind_core::timeutil::{now_ms, to_iso};
use localgrind_core::types::{Comparator, Difficulty, Signature};
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub mod highlight;

pub type Result<T> = std::result::Result<T, String>;

pub const DEFAULT_TIME_LIMIT_MS: u64 = 3000;
/// Minimum suite size (examples + generated) so judging is meaningful.
pub const MIN_TESTS: usize = 30;
const EXAMPLES_MARKER: &str = "<!-- examples -->";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExampleFile {
    pub input: String,
    pub output: String,
    #[serde(default)]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProblemFile {
    pub id: i64,
    pub title: String,
    pub difficulty: String,
    pub topics: Vec<String>,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default)]
    pub time_limit_ms: Option<u64>,
    pub signature: Signature,
    #[serde(default)]
    pub judge: Comparator,
    pub examples: Vec<ExampleFile>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListFile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub position: usize,
    /// `[group, slug]` pairs in display order.
    pub items: Vec<(String, String)>,
}

/// Loaded and validated source for one problem.
#[derive(Debug, Clone)]
pub struct Source {
    pub slug: String,
    pub dir: PathBuf,
    pub meta: ProblemFile,
    pub difficulty: Difficulty,
    pub description_md: String,
    pub editorial_md: String,
    pub solution: String,
    /// Optional reference solutions in other languages (`solution.js`, `solution.cpp`, ...).
    pub extra_solutions: BTreeMap<Language, String>,
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

pub fn load_problem(dir: &Path) -> Result<Source> {
    let slug = dir
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("bad directory {}", dir.display()))?
        .to_string();
    let meta: ProblemFile = toml::from_str(&read(&dir.join("problem.toml"))?)
        .map_err(|e| format!("{slug}/problem.toml: {e}"))?;
    let difficulty = meta
        .difficulty
        .parse()
        .map_err(|e| format!("{slug}: {e}"))?;
    meta.signature
        .validate()
        .map_err(|e| format!("{slug}: {e}"))?;
    if let Comparator::Validator { name } = &meta.judge {
        if !judge::validators::exists(name) {
            return Err(format!("{slug}: unknown validator `{name}`"));
        }
    }
    if meta.examples.is_empty() {
        return Err(format!("{slug}: at least one example is required"));
    }
    let src = Source {
        description_md: read(&dir.join("description.md"))?,
        editorial_md: read(&dir.join("editorial.md"))?,
        solution: read(&dir.join("solution.py"))?,
        extra_solutions: Language::ALL
            .iter()
            .filter(|l| **l != Language::Python)
            .filter_map(|&l| {
                let path = dir.join(format!("solution.{}", l.extension()));
                path.is_file().then(|| read(&path).map(|code| (l, code)))
            })
            .collect::<Result<_>>()?,
        slug,
        dir: dir.to_path_buf(),
        meta,
        difficulty,
    };
    if !src.dir.join("gen.py").is_file() {
        return Err(format!("{}: gen.py is missing", src.slug));
    }
    Ok(src)
}

pub fn load_all(content: &Path) -> Result<Vec<Source>> {
    let dir = content.join("problems");
    let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("problem.toml").is_file())
        .collect();
    entries.sort();
    let sources: Vec<Source> = entries
        .iter()
        .map(|p| load_problem(p))
        .collect::<Result<_>>()?;
    let mut ids = HashSet::new();
    for s in &sources {
        if !ids.insert(s.meta.id) {
            return Err(format!("duplicate problem id {} ({})", s.meta.id, s.slug));
        }
    }
    Ok(sources)
}

/// Loads the named problems only (all problems when `slugs` is empty), so one problem can be
/// checked while others in the tree are half-written.
pub fn load_selected(content: &Path, slugs: &[String]) -> Result<Vec<Source>> {
    if slugs.is_empty() {
        return load_all(content);
    }
    slugs
        .iter()
        .map(|slug| {
            let dir = content.join("problems").join(slug);
            if dir.join("problem.toml").is_file() {
                load_problem(&dir)
            } else {
                Err(format!("no problem `{slug}`"))
            }
        })
        .collect()
}

/// Renders markdown to HTML. Inside code spans, `^N` becomes a superscript so constraints such
/// as `1 <= n <= 10^4` read naturally.
pub fn render_markdown(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let mut events = Vec::new();
    // Fenced code blocks are highlighted here so the app only displays static HTML.
    let mut code: Option<(String, String)> = None;
    for ev in Parser::new_ext(md, opts) {
        match ev {
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(l) => l.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                code = Some((lang, String::new()));
            }
            Event::Text(t) if code.is_some() => code.as_mut().unwrap().1.push_str(&t),
            Event::End(TagEnd::CodeBlock) => {
                let (lang, text) = code.take().unwrap_or_default();
                events.push(Event::Html(CowStr::from(highlight::highlight(
                    &text, &lang,
                ))));
            }
            Event::Code(c) => events.push(Event::InlineHtml(CowStr::from(format!(
                "<code>{}</code>",
                superscripts(&escape(&c))
            )))),
            other => events.push(other),
        }
    }
    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    out
}

pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn superscripts(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '^' {
            let mut j = i + 1;
            if j < chars.len() && chars[j] == '-' {
                j += 1;
            }
            let digits_start = j;
            while j < chars.len() && chars[j].is_ascii_digit() {
                j += 1;
            }
            if j > digits_start {
                out.push_str("<sup>");
                out.extend(&chars[i + 1..j]);
                out.push_str("</sup>");
                i = j;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// One-line rendering of an example's input: `nums = [2,7,11,15], target = 9`.
pub fn example_input_line(sig: &Signature, input: &str) -> Result<String> {
    let values: Vec<&str> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let names: Vec<&str> = match sig {
        Signature::Function { params, .. } => params.iter().map(|p| p.name.as_str()).collect(),
        Signature::Codec { encode, .. } => vec![encode.params[0].name.as_str()],
        Signature::Design { .. } => return Ok(values.join("\n")),
    };
    if values.len() != names.len() {
        return Err(format!(
            "example has {} lines, expected {}",
            values.len(),
            names.len()
        ));
    }
    Ok(names
        .iter()
        .zip(values)
        .map(|(n, v)| format!("{n} = {v}"))
        .collect::<Vec<_>>()
        .join(", "))
}

pub fn render_examples(sig: &Signature, examples: &[ExampleFile]) -> Result<String> {
    let mut out = String::new();
    for (i, ex) in examples.iter().enumerate() {
        out.push_str(&format!(
            "<p><strong class=\"example\">Example {}:</strong></p>\n<pre class=\"example-block\"><strong>Input:</strong> {}\n<strong>Output:</strong> {}",
            i + 1,
            escape(&example_input_line(sig, &ex.input)?),
            escape(ex.output.trim())
        ));
        if let Some(exp) = &ex.explanation {
            out.push_str(&format!(
                "\n<strong>Explanation:</strong> {}",
                escape(exp.trim())
            ));
        }
        out.push_str("</pre>\n");
    }
    Ok(out)
}

pub fn render_description(src: &Source) -> Result<String> {
    let examples = render_examples(&src.meta.signature, &src.meta.examples)?;
    let (before, after) = match src.description_md.split_once(EXAMPLES_MARKER) {
        Some((b, a)) => (b, a),
        None => (src.description_md.as_str(), ""),
    };
    Ok(format!(
        "{}{}{}",
        render_markdown(before),
        examples,
        render_markdown(after)
    ))
}

/// Parses the examples' input text into argument arrays.
pub fn example_args(src: &Source) -> Result<Vec<Value>> {
    src.meta
        .examples
        .iter()
        .enumerate()
        .map(|(i, ex)| {
            let values: Vec<Value> = ex
                .input
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(serde_json::from_str)
                .collect::<std::result::Result<_, _>>()
                .map_err(|e| format!("{}: example {} input: {e}", src.slug, i + 1))?;
            let args = Value::Array(values);
            src.meta
                .signature
                .check_args(&args)
                .map_err(|e| format!("{}: example {}: {e}", src.slug, i + 1))?;
            Ok(args)
        })
        .collect()
}

/// How long a generator may run; a hung `gen.py` fails the build instead of stalling it.
const GENERATOR_TIMEOUT: Duration = Duration::from_secs(120);

/// Runs `gen.py` and returns its argument arrays.
pub fn generated_args(src: &Source, tools: &Toolchains) -> Result<Vec<Value>> {
    generated_args_within(src, tools, GENERATOR_TIMEOUT)
}

fn generated_args_within(
    src: &Source,
    tools: &Toolchains,
    timeout: Duration,
) -> Result<Vec<Value>> {
    let out = run_with_timeout(
        Command::new(tools.path(Language::Python))
            .arg("gen.py")
            // Pin set/dict iteration order so builds are reproducible.
            .env("PYTHONHASHSEED", "0")
            .current_dir(&src.dir),
        timeout,
    )
    .map_err(|e| format!("{}: running gen.py: {e}", src.slug))?
    .ok_or_else(|| format!("{}: gen.py did not finish within {timeout:?}", src.slug))?;
    if !out.status.success() {
        return Err(format!(
            "{}: gen.py failed:\n{}",
            src.slug,
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .map(|(i, l)| {
            let v: Value = serde_json::from_str(l)
                .map_err(|e| format!("{}: gen.py line {}: {e}", src.slug, i + 1))?;
            src.meta
                .signature
                .check_args(&v)
                .map_err(|e| format!("{}: gen.py line {}: {e}", src.slug, i + 1))?;
            Ok(v)
        })
        .collect()
}

/// Runs `cmd` like [`Command::output`], killing it after `timeout` (then `Ok(None)`).
fn run_with_timeout(cmd: &mut Command, timeout: Duration) -> std::io::Result<Option<Output>> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let drain = |mut pipe: Box<dyn Read + Send>| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            let _ = pipe.read_to_end(&mut buf);
            buf
        })
    };
    let stdout = drain(Box::new(child.stdout.take().expect("piped")));
    let stderr = drain(Box::new(child.stderr.take().expect("piped")));
    let deadline = Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    Ok(Some(Output {
        status,
        stdout: stdout.join().unwrap_or_default(),
        stderr: stderr.join().unwrap_or_default(),
    }))
}

fn reference_limits() -> RunOptions {
    RunOptions {
        limits: Limits {
            time_limit_ms: 120_000,
            wall_limit_ms: 300_000,
            memory_limit_kb: 8 * 1024 * 1024,
            compile_timeout_ms: 60_000,
        },
        stop_on_failure: false,
        keep_all_cases: true,
        low_priority: false,
        cache_dir: None,
        collect_only: false,
    }
}

/// Runs the reference solution over `args`, returning one output per argument array.
pub fn reference_outputs(src: &Source, args: &[Value], tools: &Toolchains) -> Result<Vec<Value>> {
    let program = Language::Python
        .driver()
        .program(&src.meta.signature, &src.solution, tools);
    let cases: Vec<TestCase> = args
        .iter()
        .map(|a| TestCase {
            args: a.clone(),
            expected: Value::Null,
        })
        .collect();
    let opts = RunOptions {
        collect_only: true,
        ..reference_limits()
    };
    let report = runner::execute(&program, &cases, &Comparator::Exact, &opts, &mut |_, _| {})
        .map_err(|e| format!("{}: {e}", src.slug))?;
    if report.cases.len() != cases.len() {
        return Err(format!(
            "{}: reference solution failed ({:?}): {}",
            src.slug,
            report.verdict,
            report.error.unwrap_or_default()
        ));
    }
    Ok(report
        .cases
        .into_iter()
        .map(|c| c.output.unwrap_or(Value::Null))
        .collect())
}

/// Fully built problem: catalog row, test suite and reference solutions by language.
pub struct Built {
    pub problem: Problem,
    pub tests: Vec<TestCase>,
    pub solutions: BTreeMap<Language, String>,
    pub benchmarks: BTreeMap<String, Benchmark>,
}

/// Whether `language`'s toolchain is usable (probed once per executable per process).
pub fn toolchain_ok(language: Language, tools: &Toolchains) -> bool {
    use std::sync::OnceLock;
    static PROBED: OnceLock<Mutex<HashMap<PathBuf, bool>>> = OnceLock::new();
    let exe = tools.path(language);
    let cache = PROBED.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(&ok) = cache.lock().unwrap().get(&exe) {
        return ok;
    }
    let ok = localgrind_core::lang::probe(language, tools).ok;
    cache.lock().unwrap().insert(exe, ok);
    ok
}

/// Runs `code` in `language` over the full suite and returns the runner's report.
fn judge_suite(
    src: &Source,
    language: Language,
    code: &str,
    tests: &[TestCase],
    opts: &RunOptions,
    tools: &Toolchains,
) -> Result<runner::RunReport> {
    let program = language.driver().program(&src.meta.signature, code, tools);
    runner::execute(&program, tests, &src.meta.judge, opts, &mut |_, _| {})
        .map_err(|e| format!("{}: {e}", src.slug))
}

/// Every non-Python reference solution must be Accepted on the generated suite. Languages
/// without an installed toolchain are skipped.
fn verify_extra_solutions(src: &Source, tests: &[TestCase], tools: &Toolchains) -> Result<()> {
    for (&language, code) in &src.extra_solutions {
        if !language.driver().supports(&src.meta.signature) {
            return Err(format!(
                "{}: solution.{} exists but {} cannot express this problem",
                src.slug,
                language.extension(),
                language.display_name()
            ));
        }
        if !toolchain_ok(language, tools) {
            continue;
        }
        let r = judge_suite(src, language, code, tests, &reference_limits(), tools)?;
        if r.verdict != Verdict::Accepted {
            let detail = r
                .failure
                .map(|f| {
                    format!(
                        " on test #{}: expected {} got {:?}",
                        f.index, f.expected, f.output
                    )
                })
                .unwrap_or_default();
            return Err(format!(
                "{}: solution.{} was not accepted: {}{detail}\n{}",
                src.slug,
                language.extension(),
                r.verdict.label(),
                r.error.unwrap_or_default()
            ));
        }
    }
    Ok(())
}

/// Builds a problem and requires every other-language reference solution to be accepted.
pub fn build_problem(src: &Source, tools: &Toolchains) -> Result<Built> {
    let built = build_suite(src, tools)?;
    verify_extra_solutions(src, &built.tests, tools)?;
    Ok(built)
}

/// Builds a problem's catalog row and test suite without judging the other-language references
/// (`judge` only needs the suite; `bench` judges each reference on every timed run anyway).
fn build_suite(src: &Source, tools: &Toolchains) -> Result<Built> {
    let examples = example_args(src)?;
    let mut args = examples.clone();
    args.extend(generated_args(src, tools)?);
    let outputs = reference_outputs(src, &args, tools)?;
    for (i, ex) in src.meta.examples.iter().enumerate() {
        let stated: Value = serde_json::from_str(ex.output.trim())
            .map_err(|e| format!("{}: example {} output: {e}", src.slug, i + 1))?;
        if !judge::matches(&src.meta.judge, &examples[i], &stated, &outputs[i]) {
            return Err(format!(
                "{}: example {} says {} but the reference solution returns {}",
                src.slug,
                i + 1,
                stated,
                outputs[i]
            ));
        }
    }
    if args.len() < MIN_TESTS {
        return Err(format!(
            "{}: only {} tests; at least {MIN_TESTS} are required",
            src.slug,
            args.len()
        ));
    }
    let tests: Vec<TestCase> = args
        .into_iter()
        .zip(outputs)
        .map(|(args, expected)| TestCase { args, expected })
        .collect();
    let bench_path = src.dir.join("benchmark.json");
    let benchmarks = if bench_path.is_file() {
        serde_json::from_str(&read(&bench_path)?)
            .map_err(|e| format!("{}: benchmark.json: {e}", src.slug))?
    } else {
        BTreeMap::new()
    };
    Ok(Built {
        problem: Problem {
            summary: ProblemSummary {
                id: src.meta.id,
                slug: src.slug.clone(),
                title: src.meta.title.clone(),
                difficulty: src.difficulty,
                topics: src.meta.topics.clone(),
            },
            signature: src.meta.signature.clone(),
            comparator: src.meta.judge.clone(),
            time_limit_ms: src.meta.time_limit_ms.unwrap_or(DEFAULT_TIME_LIMIT_MS),
            description_html: render_description(src)?,
            editorial_html: render_markdown(&src.editorial_md),
            hints: src.meta.hints.clone(),
            examples: src
                .meta
                .examples
                .iter()
                .map(|e| Example {
                    input: e.input.trim().to_string(),
                    output: e.output.trim().to_string(),
                    explanation: e.explanation.clone(),
                })
                .collect(),
            test_count: tests.len(),
        },
        tests,
        solutions: std::iter::once((Language::Python, src.solution.clone()))
            .chain(src.extra_solutions.clone())
            .collect(),
        benchmarks,
    })
}

pub fn load_lists(content: &Path, slugs: &HashMap<String, i64>) -> Result<Vec<ProblemList>> {
    let dir = content.join("lists");
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    paths.sort();
    let mut lists: Vec<(usize, ProblemList)> = Vec::new();
    for p in paths {
        let f: ListFile =
            toml::from_str(&read(&p)?).map_err(|e| format!("{}: {e}", p.display()))?;
        // A typo would otherwise silently drop a problem from the list.
        let unknown: Vec<&str> = f
            .items
            .iter()
            .filter(|(_, slug)| !slugs.contains_key(slug))
            .map(|(_, slug)| slug.as_str())
            .collect();
        if !unknown.is_empty() {
            return Err(format!(
                "{}: unknown problem slugs: {}",
                p.display(),
                unknown.join(", ")
            ));
        }
        let items = f
            .items
            .iter()
            .map(|(group, slug)| ListItem {
                problem_id: slugs[slug],
                group: group.clone(),
            })
            .collect();
        lists.push((
            f.position,
            ProblemList {
                id: f.id,
                name: f.name,
                description: f.description,
                items,
            },
        ));
    }
    lists.sort_by_key(|(position, _)| *position);
    Ok(lists.into_iter().map(|(_, l)| l).collect())
}

/// `content/companies.json`: `{ "<slug>": [{company, frequency, recency}, ...] }`.
pub fn load_companies(content: &Path) -> Result<BTreeMap<String, Vec<CompanyTag>>> {
    let path = content.join("companies.json");
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }
    serde_json::from_str(&read(&path)?).map_err(|e| format!("companies.json: {e}"))
}

#[derive(Debug, Default, Serialize)]
pub struct BuildReport {
    pub problems: usize,
    pub tests: usize,
    pub lists: usize,
    pub company_tags: usize,
}

/// Builds every problem (in parallel) and writes the catalog to `out`.
/// [`build_problem`] for each source on `jobs` threads; results are in source order.
fn build_all(sources: &[Source], tools: &Toolchains, jobs: usize) -> Vec<Result<Built>> {
    let results: Mutex<Vec<(usize, Result<Built>)>> = Mutex::new(Vec::new());
    let next = Mutex::new(0usize);
    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            s.spawn(|| loop {
                let i = {
                    let mut n = next.lock().unwrap();
                    let i = *n;
                    *n += 1;
                    i
                };
                let Some(src) = sources.get(i) else { break };
                let r = build_problem(src, tools);
                results.lock().unwrap().push((i, r));
            });
        }
    });
    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|(i, _)| *i);
    results.into_iter().map(|(_, r)| r).collect()
}

pub fn build(
    content: &Path,
    out: &Path,
    tools: &Toolchains,
    jobs: usize,
    built_at: &str,
) -> Result<BuildReport> {
    let sources = load_all(content)?;
    let results = build_all(&sources, tools, jobs);
    let errors: Vec<String> = results
        .iter()
        .filter_map(|r| r.as_ref().err().cloned())
        .collect();
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let w = CatalogWriter::create(out).map_err(|e| e.to_string())?;
    w.set_meta("built_at", built_at)
        .map_err(|e| e.to_string())?;
    let mut report = BuildReport::default();
    let mut slugs = HashMap::new();
    for r in results {
        let b = r.expect("errors handled above");
        let id = b.problem.summary.id;
        slugs.insert(b.problem.summary.slug.clone(), id);
        w.add_problem(&b.problem, &b.tests)
            .map_err(|e| e.to_string())?;
        for (&lang, code) in &b.solutions {
            w.add_solution(id, lang, code, &highlight::highlight(code, lang.id()))
                .map_err(|e| e.to_string())?;
        }
        for (lang, bench) in &b.benchmarks {
            w.add_benchmark(id, lang, bench)
                .map_err(|e| e.to_string())?;
        }
        report.problems += 1;
        report.tests += b.tests.len();
    }
    let lists = load_lists(content, &slugs)?;
    for (i, l) in lists.iter().enumerate() {
        w.add_list(l, i).map_err(|e| e.to_string())?;
    }
    report.lists = lists.len();
    let companies = load_companies(content)?;
    let unknown: Vec<&str> = companies
        .keys()
        .filter(|slug| !slugs.contains_key(*slug))
        .map(String::as_str)
        .collect();
    if !unknown.is_empty() {
        return Err(format!(
            "companies.json: unknown problem slugs: {}",
            unknown.join(", ")
        ));
    }
    for (slug, tags) in &companies {
        for t in tags {
            w.add_company(slugs[slug], t).map_err(|e| e.to_string())?;
            report.company_tags += 1;
        }
    }
    w.finish().map_err(|e| e.to_string())?;
    Ok(report)
}

/// Judges arbitrary code against a problem's full suite with the app's default limits, as a
/// submission would be. Used to confirm that naive or buggy solutions are rejected.
pub fn judge_file(
    src: &Source,
    language: Language,
    code: &str,
    tools: &Toolchains,
) -> Result<String> {
    let built = build_suite(src, tools)?;
    let time = built.problem.time_limit_ms;
    let opts = RunOptions {
        limits: Limits {
            time_limit_ms: time,
            wall_limit_ms: time * 3 + 10_000,
            ..Limits::default()
        },
        stop_on_failure: true,
        keep_all_cases: false,
        low_priority: false,
        cache_dir: None,
        collect_only: false,
    };
    let r = judge_suite(src, language, code, &built.tests, &opts, tools)?;
    let mut out = format!(
        "{} ({}/{} passed, {:.1} ms, {} KB)",
        r.verdict.label(),
        r.passed,
        r.total,
        r.runtime_ms,
        r.memory_kb
    );
    if let Some(f) = &r.failure {
        out.push_str(&format!(
            "\nfailing test #{}: args={} expected={} output={}",
            f.index,
            truncate(&f.args.to_string()),
            truncate(&f.expected.to_string()),
            f.output
                .as_ref()
                .map(|o| truncate(&o.to_string()))
                .unwrap_or_default()
        ));
    }
    if let Some(e) = &r.error {
        out.push_str(&format!("\n{e}"));
    }
    Ok(out)
}

fn truncate(s: &str) -> String {
    if s.len() <= 300 {
        s.to_string()
    } else {
        let mut end = 300;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}… ({} chars)", &s[..end], s.len())
    }
}

/// Runs each available reference solution `runs` times over the full suite and returns the
/// averages by language.
pub fn benchmark(
    src: &Source,
    tools: &Toolchains,
    runs: u32,
) -> Result<BTreeMap<Language, Benchmark>> {
    let built = build_suite(src, tools)?;
    let mut out = BTreeMap::new();
    for (&language, code) in &built.solutions {
        if toolchain_ok(language, tools) {
            out.insert(
                language,
                benchmark_one(src, language, code, &built.tests, tools, runs)?,
            );
        }
    }
    Ok(out)
}

fn benchmark_one(
    src: &Source,
    language: Language,
    code: &str,
    tests: &[TestCase],
    tools: &Toolchains,
    runs: u32,
) -> Result<Benchmark> {
    let mut opts = reference_limits();
    opts.keep_all_cases = false;
    // Compile once; every timed run reuses the cached binary. Removed on every exit path.
    let cache = tempfile::Builder::new()
        .prefix("lg-bench-")
        .tempdir()
        .map_err(|e| e.to_string())?;
    opts.cache_dir = Some(cache.path().to_path_buf());
    let (mut runtime, mut memory) = (0.0, 0u64);
    for _ in 0..runs.max(1) {
        let r = judge_suite(src, language, code, tests, &opts, tools)?;
        if r.verdict != Verdict::Accepted {
            return Err(format!(
                "{}: reference solution was not accepted: {:?}",
                src.slug, r.verdict
            ));
        }
        runtime += r.runtime_ms;
        memory += r.memory_kb;
    }
    let runs = runs.max(1);
    Ok(Benchmark {
        runtime_ms: (runtime / runs as f64 * 100.0).round() / 100.0,
        memory_kb: memory / runs as u64,
        runs,
        local: false,
    })
}

/// Command-line entry point; returns the message to print on success.
pub const USAGE: &str = "usage:
  lg-content build [--content DIR] [--out FILE] [--jobs N]
  lg-content bench [--content DIR] [--runs N] [SLUG...]
  lg-content check [--content DIR] [--jobs N] [SLUG...]
  lg-content judge [--content DIR] SLUG SOLUTION.{py,js,cpp,rs}
  lg-content starter [--content DIR] SLUG LANGUAGE";

pub fn cli(args: &[String]) -> Result<String> {
    let (cmd, rest) = args.split_first().ok_or(USAGE)?;
    let mut content = PathBuf::from("content");
    let mut out = PathBuf::from("assets/problems.db");
    let mut jobs = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let mut runs = 10u32;
    let mut slugs = Vec::new();
    let mut it = rest.iter();
    while let Some(a) = it.next() {
        let mut value = || it.next().cloned().ok_or(format!("{a} needs a value"));
        match a.as_str() {
            "--content" => content = PathBuf::from(value()?),
            "--out" => out = PathBuf::from(value()?),
            "--jobs" => jobs = value()?.parse().map_err(|_| "bad --jobs")?,
            "--runs" => runs = value()?.parse().map_err(|_| "bad --runs")?,
            s if s.starts_with("--") => return Err(format!("unknown flag {s}\n{USAGE}")),
            s => slugs.push(s.to_string()),
        }
    }
    let tools = Toolchains::detect();
    match cmd.as_str() {
        "build" => {
            let r = build(&content, &out, &tools, jobs, &to_iso(now_ms()))?;
            Ok(format!(
                "built {} problems, {} tests, {} lists, {} company tags -> {}",
                r.problems,
                r.tests,
                r.lists,
                r.company_tags,
                out.display()
            ))
        }
        "check" => {
            let sources = load_selected(&content, &slugs)?;
            if sources.is_empty() {
                return Err("no problems found".into());
            }
            let mut lines = Vec::new();
            let mut errors = Vec::new();
            for (src, r) in sources.iter().zip(build_all(&sources, &tools, jobs)) {
                match r {
                    Ok(b) => {
                        lines.push(format!("ok   {:<50} {:>4} tests", src.slug, b.tests.len()))
                    }
                    Err(e) => errors.push(e),
                }
            }
            if errors.is_empty() {
                Ok(lines.join("\n"))
            } else {
                Err(errors.join("\n"))
            }
        }
        "judge" => {
            let [slug, file] = slugs.as_slice() else {
                return Err(USAGE.into());
            };
            let src = load_selected(&content, std::slice::from_ref(slug))?.remove(0);
            let path = Path::new(file);
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or_default();
            let language = Language::from_extension(ext)
                .ok_or(format!("unknown solution language `.{ext}`"))?;
            judge_file(&src, language, &read(path)?, &tools)
        }
        "starter" => {
            let [slug, lang] = slugs.as_slice() else {
                return Err(USAGE.into());
            };
            let language: Language = lang.parse().map_err(|e| format!("{e}"))?;
            let src = load_selected(&content, std::slice::from_ref(slug))?.remove(0);
            if !language.driver().supports(&src.meta.signature) {
                return Err(format!("{} cannot express {slug}", language.display_name()));
            }
            Ok(language.driver().starter_code(&src.meta.signature))
        }
        "bench" => {
            let mut done = 0;
            for src in load_selected(&content, &slugs)? {
                let path = src.dir.join("benchmark.json");
                let mut all: BTreeMap<String, serde_json::Value> = std::fs::read_to_string(&path)
                    .ok()
                    .and_then(|s| serde_json::from_str(&s).ok())
                    .unwrap_or_default();
                for (language, b) in benchmark(&src, &tools, runs)? {
                    all.insert(language.id().into(), serde_json::to_value(&b).unwrap());
                    println!(
                        "{:<50} {:<10} {:>9.2} ms {:>8} KB",
                        src.slug,
                        language.id(),
                        b.runtime_ms,
                        b.memory_kb
                    );
                }
                std::fs::write(&path, serde_json::to_string_pretty(&all).unwrap() + "\n")
                    .map_err(|e| e.to_string())?;
                done += 1;
            }
            Ok(format!("benchmarked {done} problems"))
        }
        _ => Err(USAGE.into()),
    }
}

#[cfg(test)]
mod tests;
