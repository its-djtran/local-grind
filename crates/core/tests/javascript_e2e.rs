//! Runs real JavaScript solutions through the generated harness and the runner.

use localgrind_core::lang::{Language, Toolchains};
use localgrind_core::runner::{execute, RunOptions, RunReport, TestCase, Verdict};
use localgrind_core::types::{Comparator, Signature};
use serde_json::{json, Value};

fn sig(v: Value) -> Signature {
    serde_json::from_value(v).unwrap()
}

fn run_opts(
    sig: &Signature,
    code: &str,
    cases: &[(Value, Value)],
    cmp: Comparator,
    opts: RunOptions,
) -> RunReport {
    let program = Language::JavaScript
        .driver()
        .program(sig, code, &Toolchains::detect());
    let cases: Vec<TestCase> = cases
        .iter()
        .map(|(a, e)| TestCase {
            args: a.clone(),
            expected: e.clone(),
        })
        .collect();
    execute(&program, &cases, &cmp, &opts, &mut |_, _| {}).unwrap()
}

fn run(sig: &Signature, code: &str, cases: &[(Value, Value)]) -> RunReport {
    let opts = RunOptions {
        keep_all_cases: true,
        ..Default::default()
    };
    run_opts(sig, code, cases, Comparator::Exact, opts)
}

fn two_sum() -> Signature {
    sig(json!({"kind": "function", "name": "twoSum",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
        "returns": "int[]"}))
}

#[test]
fn accepted_with_stdout() {
    let code = "
var twoSum = function(nums, target) {
    console.log('looking for', target);
    const seen = new Map();
    for (let i = 0; i < nums.length; i++) {
        if (seen.has(target - nums[i])) return [seen.get(target - nums[i]), i];
        seen.set(nums[i], i);
    }
};
";
    let r = run(
        &two_sum(),
        code,
        &[
            (json!([[2, 7, 11, 15], 9]), json!([0, 1])),
            (json!([[3, 2, 4], 6]), json!([1, 2])),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
    assert_eq!(r.cases[1].stdout, "looking for 6\n");
    assert!(r.memory_kb > 1000);
}

#[test]
fn wrong_answer() {
    let r = run(
        &two_sum(),
        "var twoSum = (nums, target) => [1, 0];",
        &[(json!([[2, 7], 9]), json!([0, 1]))],
    );
    assert_eq!(r.verdict, Verdict::WrongAnswer);
    assert_eq!(r.failure.unwrap().output, Some(json!([1, 0])));
}

#[test]
fn syntax_error_is_compile_error() {
    let r = run(
        &two_sum(),
        "var twoSum = function(nums, target) {\n  return [;\n};\n",
        &[(json!([[1], 1]), json!([]))],
    );
    assert_eq!(r.verdict, Verdict::CompileError);
    let err = r.error.unwrap();
    assert!(
        err.contains("solution.js:2") && err.contains("SyntaxError"),
        "{err}"
    );
}

#[test]
fn runtime_errors_show_user_frames() {
    let code = "var twoSum = function(nums, target) {\n  return nums.missing.length;\n};\n";
    let r = run(&two_sum(), code, &[(json!([[1], 1]), json!([]))]);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let err = r.error.unwrap();
    assert!(
        err.contains("TypeError") && err.contains("solution.js:2"),
        "{err}"
    );
    assert!(!err.contains("lg_harness"), "{err}");

    let r = run(
        &two_sum(),
        "throw 'top-level';",
        &[(json!([[1], 1]), json!([]))],
    );
    assert!(r.error.unwrap().contains("Uncaught 'top-level'"));

    let r = run(
        &two_sum(),
        "var other = 1;",
        &[(json!([[1], 1]), json!([]))],
    );
    assert!(r.error.unwrap().contains("twoSum is not defined"));
}

#[test]
fn infinite_loop_times_out() {
    let mut opts = RunOptions::default();
    opts.limits.wall_limit_ms = 1500;
    let r = run_opts(
        &two_sum(),
        "var twoSum = () => { while (true) {} };",
        &[(json!([[1], 1]), json!([]))],
        Comparator::Exact,
        opts,
    );
    assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
}

#[test]
fn lists_trees_and_deep_recursion() {
    let reverse = sig(json!({"kind": "function", "name": "reverseList",
        "params": [{"name": "head", "type": "ListNode"}], "returns": "ListNode"}));
    let code = "var reverseList = function(head) { let p = null; while (head) { const n = head.next; head.next = p; p = head; head = n; } return p; };";
    let r = run(
        &reverse,
        code,
        &[
            (json!([[1, 2, 3]]), json!([3, 2, 1])),
            (json!([[]]), json!([])),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let invert = sig(json!({"kind": "function", "name": "invertTree",
        "params": [{"name": "root", "type": "TreeNode"}], "returns": "TreeNode"}));
    let code = "var invertTree = function(root) { if (root) { [root.left, root.right] = [invertTree(root.right), invertTree(root.left)]; } return root; };";
    let mut spine = vec![json!(0)];
    let mut mirrored = vec![json!(0)];
    for i in 1..20_000 {
        spine.extend([json!(i), Value::Null]);
        mirrored.extend([Value::Null, json!(i)]);
    }
    let r = run(
        &invert,
        code,
        &[
            (json!([[4, 2, 7, 1, 3, 6, 9]]), json!([4, 7, 2, 9, 6, 3, 1])),
            (json!([spine]), Value::Array(mirrored)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{:?}", r.error);

    let cyclic = sig(json!({"kind": "function", "name": "f",
        "params": [{"name": "head", "type": "ListNode"}], "returns": "ListNode"}));
    let r = run(
        &cyclic,
        "var f = h => { h.next = h; return h; };",
        &[(json!([[1, 2]]), json!([]))],
    );
    assert!(r.error.unwrap().contains("cycle"));
}

#[test]
fn graphs_with_deep_copy() {
    let s = sig(
        json!({"kind": "function", "name": "cloneGraph", "special": "deep_copy",
        "params": [{"name": "node", "type": "Node"}], "returns": "Node"}),
    );
    let good = "
var cloneGraph = function(node) {
    if (!node) return null;
    const copies = new Map();
    const dfs = (n) => {
        if (copies.has(n)) return copies.get(n);
        const c = new _Node(n.val);
        copies.set(n, c);
        c.neighbors = n.neighbors.map(dfs);
        return c;
    };
    return dfs(node);
};
";
    let adj = json!([[2, 4], [1, 3], [2, 4], [1, 3]]);
    let r = run(
        &s,
        good,
        &[
            (json!([adj.clone()]), adj.clone()),
            (json!([[]]), json!([])),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
    let r = run(
        &s,
        "var cloneGraph = n => n;",
        &[(json!([adj.clone()]), adj)],
    );
    assert!(r.error.unwrap().contains("deep copy"));
}

#[test]
fn in_place_doubles_and_floats() {
    let sort = sig(json!({"kind": "function", "name": "sortColors",
        "params": [{"name": "nums", "type": "int[]"}], "returns": "void", "output_param": "nums"}));
    let r = run(
        &sort,
        "var sortColors = nums => { nums.sort((a, b) => a - b); };",
        &[(json!([[2, 0, 1]]), json!([0, 1, 2]))],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let avg = sig(json!({"kind": "function", "name": "avg",
        "params": [{"name": "nums", "type": "int[]"}], "returns": "double"}));
    let r = run_opts(
        &avg,
        "var avg = nums => nums.reduce((a, b) => a + b, 0) / nums.length;",
        &[(json!([[1, 2]]), json!(1.5)), (json!([[2, 2]]), json!(2.0))],
        Comparator::Float { tolerance: 1e-5 },
        RunOptions::default(),
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn design_problems_in_both_styles() {
    let s = sig(
        json!({"kind": "design", "class": "MinStack", "constructor": [],
        "methods": [
            {"name": "push", "params": [{"name": "val", "type": "int"}], "returns": "void"},
            {"name": "pop", "returns": "void"},
            {"name": "top", "returns": "int"},
            {"name": "getMin", "returns": "int"}]}),
    );
    let ops = json!(["MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin"]);
    let args = json!([[], [-2], [0], [-3], [], [], [], []]);
    let expected = json!([null, null, null, null, -3, null, 0, -2]);
    let prototype = "
var MinStack = function() { this.s = []; };
MinStack.prototype.push = function(v) { this.s.push([v, this.s.length ? Math.min(v, this.s[this.s.length - 1][1]) : v]); };
MinStack.prototype.pop = function() { this.s.pop(); };
MinStack.prototype.top = function() { return this.s[this.s.length - 1][0]; };
MinStack.prototype.getMin = function() { return this.s[this.s.length - 1][1]; };
";
    let class = "
class MinStack {
  constructor() { this.s = []; }
  push(v) { this.s.push([v, this.s.length ? Math.min(v, this.s.at(-1)[1]) : v]); }
  pop() { this.s.pop(); }
  top() { return this.s.at(-1)[0]; }
  getMin() { return this.s.at(-1)[1]; }
}
";
    for code in [prototype, class] {
        let r = run(
            &s,
            code,
            &[(json!([ops.clone(), args.clone()]), expected.clone())],
        );
        assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
    }
}

#[test]
fn codec_round_trip() {
    let s = sig(json!({"kind": "codec", "class": "Codec",
        "encode": {"name": "encode", "params": [{"name": "strs", "type": "string[]"}], "returns": "string"},
        "decode": {"name": "decode", "params": [{"name": "s", "type": "string"}], "returns": "string[]"}}));
    let good = "var encode = strs => JSON.stringify(strs);\nvar decode = s => JSON.parse(s);";
    let cases = [(json!([["a#b", ""]]), json!(["a#b", ""]))];
    assert_eq!(run(&s, good, &cases).verdict, Verdict::Accepted);
    let bad = "var encode = strs => strs;\nvar decode = s => s;";
    assert!(run(&s, bad, &cases)
        .error
        .unwrap()
        .contains("must return a string"));
}

#[test]
fn specials_node_refs_cycles_and_bad_version() {
    let lca = sig(json!({"kind": "function", "name": "lowestCommonAncestor",
        "params": [{"name": "root", "type": "TreeNode"},
                   {"name": "p", "type": "TreeNode", "ref_of": "root"},
                   {"name": "q", "type": "TreeNode", "ref_of": "root"}],
        "returns": "TreeNode", "output_mode": "node_val"}));
    let code = "
var lowestCommonAncestor = function(root, p, q) {
    while (root) {
        if (p.val < root.val && q.val < root.val) root = root.left;
        else if (p.val > root.val && q.val > root.val) root = root.right;
        else return root;
    }
};
";
    let tree = json!([6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]);
    let r = run(
        &lca,
        code,
        &[
            (json!([tree.clone(), 2, 8]), json!(6)),
            (json!([tree, 2, 4]), json!(2)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let cycle = sig(
        json!({"kind": "function", "name": "hasCycle", "special": "list_cycle",
        "params": [{"name": "head", "type": "ListNode"}, {"name": "pos", "type": "int", "hidden": true}],
        "returns": "bool"}),
    );
    let code = "var hasCycle = function(head) { let s = head, f = head; while (f && f.next) { s = s.next; f = f.next.next; if (s === f) return true; } return false; };";
    let r = run(
        &cycle,
        code,
        &[
            (json!([[3, 2, 0, -4], 1]), json!(true)),
            (json!([[1], -1]), json!(false)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let bad = sig(
        json!({"kind": "function", "name": "firstBadVersion", "special": "bad_version",
        "params": [{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}],
        "returns": "int"}),
    );
    let code = "
var solution = function(isBadVersion) {
    return function(n) {
        let lo = 1, hi = n;
        while (lo < hi) { const mid = lo + Math.floor((hi - lo) / 2); if (isBadVersion(mid)) hi = mid; else lo = mid + 1; }
        return lo;
    };
};
";
    let r = run(
        &bad,
        code,
        &[
            (json!([5, 4]), json!(4)),
            (json!([2147483647, 2]), json!(2)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}
