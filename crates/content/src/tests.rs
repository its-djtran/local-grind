use super::*;
use localgrind_core::catalog::Catalog;
use std::fs;

fn repo_content() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content")
}

/// A content tree in a temp dir holding a copy of Two Sum plus the shared generator library.
fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("lib")).unwrap();
    fs::copy(
        repo_content().join("lib/lg_gen.py"),
        root.join("lib/lg_gen.py"),
    )
    .unwrap();
    copy_problem(root, "two-sum", "two-sum");
    dir
}

fn copy_problem(root: &Path, from: &str, to: &str) -> PathBuf {
    let src = repo_content().join("problems").join(from);
    let dst = root.join("problems").join(to);
    fs::create_dir_all(&dst).unwrap();
    for f in [
        "problem.toml",
        "description.md",
        "editorial.md",
        "solution.py",
        "gen.py",
    ] {
        fs::copy(src.join(f), dst.join(f)).unwrap();
    }
    dst
}

fn edit(path: &Path, from: &str, to: &str) {
    let s = fs::read_to_string(path).unwrap();
    assert!(s.contains(from), "{from} not in {}", path.display());
    fs::write(path, s.replacen(from, to, 1)).unwrap();
}

fn tools() -> Toolchains {
    Toolchains::detect()
}

#[test]
fn markdown_rendering() {
    let html = render_markdown(
        "Use `1 <= n <= 10^4` and `x^-2` but `a^b`.\n\n| a | b |\n|---|---|\n| 1 | 2 |",
    );
    assert!(
        html.contains("<code>1 &lt;= n &lt;= 10<sup>4</sup></code>"),
        "{html}"
    );
    assert!(html.contains("<code>x<sup>-2</sup></code>"));
    assert!(html.contains("<code>a^b</code>"));
    assert!(html.contains("<table>"));
    assert_eq!(escape("<a&b>"), "&lt;a&amp;b&gt;");
}

#[test]
fn builds_catalog_from_content() {
    let dir = fixture();
    let root = dir.path();
    fs::create_dir_all(root.join("lists")).unwrap();
    fs::write(
        root.join("lists/a.toml"),
        "id = \"l1\"\nname = \"List\"\ndescription = \"d\"\nposition = 1\nitems = [[\"Array\", \"two-sum\"], [\"X\", \"missing\"]]\n",
    )
    .unwrap();
    fs::write(root.join("lists/readme.txt"), "ignored").unwrap();
    fs::write(
        root.join("companies.json"),
        r#"{"two-sum": [{"company": "Google", "frequency": 90.5, "recency": "30d"}], "nope": [{"company": "X", "frequency": 1, "recency": "all"}]}"#,
    )
    .unwrap();
    fs::write(
        root.join("problems/two-sum/benchmark.json"),
        r#"{"python": {"runtime_ms": 3.5, "memory_kb": 9000, "runs": 10}}"#,
    )
    .unwrap();
    let out = root.join("out/problems.db");
    let report = build(root, &out, &tools(), 2, "2026-10-07T00:00:00.000Z").unwrap();
    assert_eq!(
        (report.problems, report.lists, report.company_tags),
        (1, 1, 1)
    );
    assert!(report.tests > 50);

    let c = Catalog::open(&out).unwrap();
    let p = c.problem(1).unwrap();
    assert_eq!(p.summary.slug, "two-sum");
    assert_eq!(p.examples.len(), 3);
    assert!(p.description_html.contains("Example 1:"));
    assert!(p
        .description_html
        .contains("nums = [2,7,11,15], target = 9"));
    assert!(p.description_html.contains("10<sup>4</sup>"));
    assert!(p.editorial_html.contains("<h2>"));
    let tests = c.tests(1).unwrap();
    assert_eq!(tests[0].expected, serde_json::json!([0, 1]));
    assert_eq!(tests.len(), p.test_count);
    assert_eq!(c.lists().unwrap()[0].items.len(), 1);
    assert_eq!(c.benchmarks(1).unwrap()["python"].runs, 10);
    assert!(c.solution(1, Language::Python).unwrap().is_some());
    assert_eq!(
        c.meta("built_at").unwrap().as_deref(),
        Some("2026-10-07T00:00:00.000Z")
    );
}

#[test]
fn benchmark_runs_reference() {
    let dir = fixture();
    let src = load_problem(&dir.path().join("problems/two-sum")).unwrap();
    let b = benchmark(&src, &tools(), 2).unwrap();
    assert_eq!(b[&Language::Python].runs, 2);
    assert!(b[&Language::Python].memory_kb > 0);
}

const JS_TWO_SUM: &str = "var twoSum = function(nums, target) {\n  const seen = new Map();\n  for (let i = 0; i < nums.length; i++) {\n    if (seen.has(target - nums[i])) return [seen.get(target - nums[i]), i];\n    seen.set(nums[i], i);\n  }\n};\n";

#[test]
fn extra_language_solutions_are_verified_and_benchmarked() {
    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    fs::write(p.join("solution.js"), JS_TWO_SUM).unwrap();
    let src = load_problem(&p).unwrap();
    assert!(src.extra_solutions.contains_key(&Language::JavaScript));
    let built = build_problem(&src, &tools()).unwrap();
    assert_eq!(built.solutions.len(), 2);
    let b = benchmark(&src, &tools(), 1).unwrap();
    assert!(b.contains_key(&Language::JavaScript) && b.contains_key(&Language::Python));

    fs::write(p.join("solution.js"), "var twoSum = () => [0, 0];").unwrap();
    let err = build_problem(&load_problem(&p).unwrap(), &tools())
        .err()
        .unwrap();
    assert!(
        err.contains("solution.js was not accepted: Wrong Answer"),
        "{err}"
    );

    // A missing toolchain skips verification instead of failing the build.
    let mut no_node = tools();
    no_node.set(Language::JavaScript, PathBuf::from("/nonexistent/node"));
    assert!(build_problem(&load_problem(&p).unwrap(), &no_node).is_ok());
}

#[test]
fn rejects_solutions_in_unsupported_languages() {
    let dir = fixture();
    let mut src = load_problem(&dir.path().join("problems/two-sum")).unwrap();
    src.meta.signature = serde_json::from_value(serde_json::json!({"kind": "function",
        "name": "cloneGraph", "special": "deep_copy", "params": [{"name": "node", "type": "Node"}],
        "returns": "Node"}))
    .unwrap();
    src.extra_solutions
        .insert(Language::Rust, "impl Solution {}".into());
    let err = verify_extra_solutions(&src, &[], &tools()).unwrap_err();
    assert!(err.contains("Rust cannot express this problem"), "{err}");
}

#[test]
fn rejects_wrong_example_output() {
    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    edit(
        &p.join("problem.toml"),
        "output = \"[0,1]\"",
        "output = \"[0,2]\"",
    );
    let src = load_problem(&p).unwrap();
    let err = build_problem(&src, &tools()).err().unwrap();
    assert!(err.contains("example 1 says"), "{err}");
}

#[test]
fn rejects_bad_examples_and_generators() {
    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    edit(&p.join("problem.toml"), "[2,7,11,15]\n9", "[2,7,11,15]");
    let err = build_problem(&load_problem(&p).unwrap(), &tools())
        .err()
        .unwrap();
    assert!(err.contains("expected 2 argument"), "{err}");

    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    edit(&p.join("problem.toml"), "[2,7,11,15]\n9", "[2,7,11,15\n9");
    assert!(build_problem(&load_problem(&p).unwrap(), &tools()).is_err());

    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    fs::write(p.join("gen.py"), "print('[1]')\n").unwrap();
    let err = build_problem(&load_problem(&p).unwrap(), &tools())
        .err()
        .unwrap();
    assert!(err.contains("gen.py line 1"), "{err}");

    fs::write(p.join("gen.py"), "print('{oops')\n").unwrap();
    assert!(build_problem(&load_problem(&p).unwrap(), &tools()).is_err());

    fs::write(p.join("gen.py"), "raise SystemExit(3)\n").unwrap();
    let err = build_problem(&load_problem(&p).unwrap(), &tools())
        .err()
        .unwrap();
    assert!(err.contains("gen.py failed"), "{err}");

    fs::write(p.join("gen.py"), "").unwrap();
    fs::write(
        p.join("solution.py"),
        "class Solution:\n    def twoSum(self, n, t):\n        raise ValueError()\n",
    )
    .unwrap();
    let err = build_problem(&load_problem(&p).unwrap(), &tools())
        .err()
        .unwrap();
    assert!(err.contains("reference solution failed"), "{err}");

    let mut broken = tools();
    broken.set(Language::Python, PathBuf::from("/nonexistent/python"));
    let err = build_problem(&load_problem(&p).unwrap(), &broken)
        .err()
        .unwrap();
    assert!(err.contains("running gen.py"), "{err}");
}

#[test]
fn load_errors() {
    let dir = fixture();
    let root = dir.path();
    let p = root.join("problems/two-sum");
    edit(
        &p.join("problem.toml"),
        "difficulty = \"Easy\"",
        "difficulty = \"Trivial\"",
    );
    assert!(load_problem(&p).unwrap_err().contains("unknown difficulty"));

    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    edit(
        &p.join("problem.toml"),
        "mode = \"unordered\"",
        "mode = \"validator\"\nname = \"nope\"",
    );
    assert!(load_problem(&p).unwrap_err().contains("unknown validator"));

    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    edit(
        &p.join("problem.toml"),
        "returns = \"int[]\"",
        "returns = \"void\"",
    );
    assert!(load_problem(&p).unwrap_err().contains("output_param"));

    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    edit(&p.join("problem.toml"), "id = 1", "id = 1\nbogus = 2");
    assert!(load_problem(&p).unwrap_err().contains("problem.toml"));

    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    let toml = fs::read_to_string(p.join("problem.toml")).unwrap();
    fs::write(
        p.join("problem.toml"),
        "examples = []\n".to_string() + toml.split("[[examples]]").next().unwrap(),
    )
    .unwrap();
    assert!(load_problem(&p)
        .unwrap_err()
        .contains("at least one example"));

    let dir = fixture();
    let p = dir.path().join("problems/two-sum");
    fs::remove_file(p.join("gen.py")).unwrap();
    assert!(load_problem(&p).unwrap_err().contains("gen.py is missing"));
    fs::remove_file(p.join("editorial.md")).unwrap();
    assert!(load_problem(&p).unwrap_err().contains("editorial.md"));

    let dir = fixture();
    copy_problem(dir.path(), "two-sum", "two-sum-copy");
    assert!(load_all(dir.path())
        .unwrap_err()
        .contains("duplicate problem id"));
    assert!(build(dir.path(), &dir.path().join("x.db"), &tools(), 1, "now").is_err());

    assert!(load_all(Path::new("/nonexistent")).is_err());
    assert!(load_problem(Path::new("/")).is_err());
}

#[test]
fn optional_inputs_absent() {
    let dir = tempfile::tempdir().unwrap();
    assert!(load_lists(dir.path(), &HashMap::new()).unwrap().is_empty());
    assert!(load_companies(dir.path()).unwrap().is_empty());
    fs::write(dir.path().join("companies.json"), "[").unwrap();
    assert!(load_companies(dir.path()).is_err());
    fs::create_dir_all(dir.path().join("lists")).unwrap();
    fs::write(dir.path().join("lists/bad.toml"), "id = 3").unwrap();
    assert!(load_lists(dir.path(), &HashMap::new()).is_err());
}

#[test]
fn design_example_lines() {
    let sig: Signature = serde_json::from_value(serde_json::json!({"kind": "design", "class": "C",
        "methods": [{"name": "m", "returns": "int"}]}))
    .unwrap();
    assert_eq!(
        example_input_line(&sig, "[\"C\",\"m\"]\n[[],[]]").unwrap(),
        "[\"C\",\"m\"]\n[[],[]]"
    );
    let codec: Signature = serde_json::from_value(serde_json::json!({"kind": "codec", "class": "Codec",
        "encode": {"name": "serialize", "params": [{"name": "root", "type": "TreeNode"}], "returns": "string"},
        "decode": {"name": "deserialize", "params": [{"name": "data", "type": "string"}], "returns": "TreeNode"}}))
    .unwrap();
    assert_eq!(example_input_line(&codec, "[1,2]").unwrap(), "root = [1,2]");
    assert!(example_input_line(&codec, "[1,2]\n3").is_err());
}

#[test]
fn repository_content_is_valid() {
    // Every problem in the repository must load; full builds run in CI via `lg-content build`.
    let sources = load_all(&repo_content()).unwrap();
    assert!(!sources.is_empty());
    for s in &sources {
        render_description(s).unwrap();
        example_args(s).unwrap();
    }
}

#[test]
fn command_line() {
    let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let dir = fixture();
    let root = dir.path().to_string_lossy().to_string();
    let out = dir.path().join("p.db").to_string_lossy().to_string();
    let msg = cli(&s(&[
        "build",
        "--content",
        &root,
        "--out",
        &out,
        "--jobs",
        "1",
    ]))
    .unwrap();
    assert!(msg.starts_with("built 1 problems"), "{msg}");
    let msg = cli(&s(&["bench", "--content", &root, "--runs", "1", "two-sum"])).unwrap();
    assert_eq!(msg, "benchmarked 1 problems");
    let bench = fs::read_to_string(dir.path().join("problems/two-sum/benchmark.json")).unwrap();
    assert!(bench.contains("\"runs\": 1"));
    assert!(cli(&s(&["bench", "--content", &root, "other"]))
        .unwrap_err()
        .contains("no problem `other`"));
    let msg = cli(&s(&["check", "--content", &root, "two-sum"])).unwrap();
    assert!(msg.starts_with("ok   two-sum"), "{msg}");
    assert!(cli(&s(&["check", "--content", &root, "missing"]))
        .unwrap_err()
        .contains("no problem `missing`"));
    // A broken sibling problem does not affect checking a named one.
    fs::create_dir_all(dir.path().join("problems/half-written")).unwrap();
    fs::write(
        dir.path().join("problems/half-written/problem.toml"),
        "id = 9",
    )
    .unwrap();
    assert!(cli(&s(&["check", "--content", &root, "two-sum"])).is_ok());
    assert!(cli(&s(&["check", "--content", &root])).is_err());
    fs::remove_dir_all(dir.path().join("problems/half-written")).unwrap();
    fs::write(dir.path().join("problems/two-sum/gen.py"), "").unwrap();
    assert!(cli(&s(&["check", "--content", &root]))
        .unwrap_err()
        .contains("at least 30"));
    let dir = fixture();
    let root = dir.path().to_string_lossy().to_string();
    let sol = dir.path().join("p/sol.py");
    fs::create_dir_all(sol.parent().unwrap()).unwrap();
    fs::copy(dir.path().join("problems/two-sum/solution.py"), &sol).unwrap();
    let sol = sol.to_string_lossy().to_string();
    let msg = cli(&s(&["judge", "--content", &root, "two-sum", &sol])).unwrap();
    assert!(msg.starts_with("Accepted"), "{msg}");
    let js = dir.path().join("p/sol.js");
    fs::write(&js, JS_TWO_SUM).unwrap();
    let msg = cli(&s(&[
        "judge",
        "--content",
        &root,
        "two-sum",
        &js.to_string_lossy(),
    ]))
    .unwrap();
    assert!(msg.starts_with("Accepted"), "{msg}");
    let weird = dir.path().join("p/sol.kt");
    fs::write(&weird, "").unwrap();
    assert!(cli(&s(&[
        "judge",
        "--content",
        &root,
        "two-sum",
        &weird.to_string_lossy()
    ]))
    .unwrap_err()
    .contains("unknown solution language"));
    fs::write(
        &sol,
        "class Solution:\n    def twoSum(self, nums, target):\n        return [0, 0]\n",
    )
    .unwrap();
    let msg = cli(&s(&["judge", "--content", &root, "two-sum", &sol])).unwrap();
    assert!(
        msg.starts_with("Wrong Answer") && msg.contains("failing test #0"),
        "{msg}"
    );
    fs::write(&sol, "class Solution:\n    def twoSum(self, nums, target):\n        raise ValueError('x' * 400)\n").unwrap();
    let msg = cli(&s(&["judge", "--content", &root, "two-sum", &sol])).unwrap();
    assert!(
        msg.starts_with("Runtime Error") && msg.contains("ValueError"),
        "{msg}"
    );
    assert!(cli(&s(&["judge", "--content", &root, "nope", &sol]))
        .unwrap_err()
        .contains("no problem"));
    assert!(cli(&s(&["judge", "--content", &root, "two-sum"])).is_err());
    assert_eq!(
        truncate(&"é".repeat(200)).chars().count(),
        150 + "… (400 chars)".chars().count()
    );
    assert!(cli(&s(&[])).is_err());
    assert!(cli(&s(&["dance"])).is_err());
    assert!(cli(&s(&["build", "--jobs"]))
        .unwrap_err()
        .contains("needs a value"));
    assert!(cli(&s(&["build", "--jobs", "x"])).is_err());
    assert!(cli(&s(&["bench", "--runs", "x"])).is_err());
    assert!(cli(&s(&["build", "--wat"]))
        .unwrap_err()
        .contains("unknown flag"));
}
