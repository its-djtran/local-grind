//! Compiles and runs real Rust solutions through the generated harness and the runner.

use localgrind_core::lang::{Language, Toolchains};
use localgrind_core::runner::{execute, RunOptions, RunReport, TestCase, Verdict};
use localgrind_core::types::{Comparator, Signature};
use serde_json::{json, Value};

fn sig(v: Value) -> Signature {
    serde_json::from_value(v).unwrap()
}

fn run(sig: &Signature, code: &str, cases: &[(Value, Value)]) -> RunReport {
    let program = Language::Rust
        .driver()
        .program(sig, code, &Toolchains::detect());
    let cases: Vec<TestCase> = cases
        .iter()
        .map(|(a, e)| TestCase {
            args: a.clone(),
            expected: e.clone(),
        })
        .collect();
    let opts = RunOptions {
        keep_all_cases: true,
        ..Default::default()
    };
    execute(&program, &cases, &Comparator::Exact, &opts, &mut |_, _| {}).unwrap()
}

fn two_sum() -> Signature {
    sig(json!({"kind": "function", "name": "twoSum",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
        "returns": "int[]"}))
}

#[test]
fn accepted_with_stdout_and_user_imports() {
    let code = "
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        println!(\"target {}\", target);
        let mut seen = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            if let Some(&j) = seen.get(&(target - n)) {
                return vec![j as i32, i as i32];
            }
            seen.insert(n, i);
        }
        vec![]
    }
}
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
    assert_eq!(r.cases[1].stdout, "target 6\n");
}

#[test]
fn compile_errors_and_panics() {
    let code = "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        nope\n    }\n}\n";
    let r = run(&two_sum(), code, &[(json!([[1], 1]), json!([]))]);
    assert_eq!(r.verdict, Verdict::CompileError);
    let err = r.error.unwrap();
    assert!(err.contains("solution.rs:3"), "{err}");

    let code = "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        vec![nums[5]]\n    }\n}\n";
    let r = run(&two_sum(), code, &[(json!([[1], 1]), json!([]))]);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let err = r.error.unwrap();
    assert!(
        err.contains("solution.rs:3") && err.contains("index out of bounds"),
        "{err}"
    );

    let overflow = "impl Solution { pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> { vec![target + i32::MAX] } }";
    let r = run(&two_sum(), overflow, &[(json!([[1], 1]), json!([]))]);
    assert!(r.error.unwrap().contains("overflow"));
}

#[test]
fn lists_trees_and_deep_recursion() {
    let reverse = sig(json!({"kind": "function", "name": "reverseList",
        "params": [{"name": "head", "type": "ListNode"}], "returns": "ListNode"}));
    let code = "
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut cur) = (None, head);
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
";
    let r = run(
        &reverse,
        code,
        &[
            (json!([[1, 2, 3]]), json!([3, 2, 1])),
            (json!([[]]), json!([])),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let depth = sig(json!({"kind": "function", "name": "maxDepth",
        "params": [{"name": "root", "type": "TreeNode"}], "returns": "int"}));
    let code = "
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                1 + Self::max_depth(n.left.clone()).max(Self::max_depth(n.right.clone()))
            }
        }
    }
}
";
    let mut spine = vec![json!(0)];
    for i in 1..50_000 {
        spine.extend([json!(i), Value::Null]);
    }
    let r = run(
        &depth,
        code,
        &[
            (json!([[3, 9, 20, null, null, 15, 7]]), json!(3)),
            (json!([spine]), json!(50_000)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{:?}", r.error);
}

#[test]
fn in_place_design_codec_and_specials() {
    let sort = sig(json!({"kind": "function", "name": "sortColors",
        "params": [{"name": "nums", "type": "int[]"}], "returns": "void", "output_param": "nums"}));
    let r = run(
        &sort,
        "impl Solution { pub fn sort_colors(nums: &mut Vec<i32>) { nums.sort(); } }",
        &[(json!([[2, 0, 1]]), json!([0, 1, 2]))],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let min_stack = sig(
        json!({"kind": "design", "class": "MinStack", "constructor": [],
        "methods": [
            {"name": "push", "params": [{"name": "val", "type": "int"}], "returns": "void"},
            {"name": "pop", "returns": "void"},
            {"name": "top", "returns": "int"},
            {"name": "getMin", "returns": "int"}]}),
    );
    let code = "
struct MinStack { s: Vec<(i32, i32)> }
impl MinStack {
    fn new() -> Self { MinStack { s: Vec::new() } }
    fn push(&mut self, val: i32) { let m = self.s.last().map_or(val, |&(_, m)| m.min(val)); self.s.push((val, m)); }
    fn pop(&mut self) { self.s.pop(); }
    fn top(&self) -> i32 { self.s.last().unwrap().0 }
    fn get_min(&self) -> i32 { self.s.last().unwrap().1 }
}
";
    let ops = json!(["MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin"]);
    let args = json!([[], [-2], [0], [-3], [], [], [], []]);
    let r = run(
        &min_stack,
        code,
        &[(
            json!([ops, args]),
            json!([null, null, null, null, -3, null, 0, -2]),
        )],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let codec = sig(json!({"kind": "codec", "class": "Codec",
        "encode": {"name": "encode", "params": [{"name": "strs", "type": "string[]"}], "returns": "string"},
        "decode": {"name": "decode", "params": [{"name": "s", "type": "string"}], "returns": "string[]"}}));
    let code = "
struct Codec;
impl Codec {
    fn new() -> Self { Codec }
    fn encode(&self, strs: Vec<String>) -> String {
        strs.iter().map(|s| format!(\"{}#{}\", s.chars().count(), s)).collect()
    }
    fn decode(&self, s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let (mut out, mut i) = (Vec::new(), 0);
        while i < chars.len() {
            let mut j = i;
            while chars[j] != '#' { j += 1; }
            let n: usize = chars[i..j].iter().collect::<String>().parse().unwrap();
            out.push(chars[j + 1..j + 1 + n].iter().collect());
            i = j + 1 + n;
        }
        out
    }
}
";
    let r = run(
        &codec,
        code,
        &[(
            json!([["lint", "co#de", "", "é\"\\"]]),
            json!(["lint", "co#de", "", "é\"\\"]),
        )],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let lca = sig(json!({"kind": "function", "name": "lowestCommonAncestor",
        "params": [{"name": "root", "type": "TreeNode"},
                   {"name": "p", "type": "TreeNode", "ref_of": "root"},
                   {"name": "q", "type": "TreeNode", "ref_of": "root"}],
        "returns": "TreeNode", "output_mode": "node_val"}));
    let code = "
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let (pv, qv) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
        let mut cur = root;
        while let Some(n) = cur.clone() {
            let v = n.borrow().val;
            if pv < v && qv < v { cur = n.borrow().left.clone(); }
            else if pv > v && qv > v { cur = n.borrow().right.clone(); }
            else { return Some(n); }
        }
        None
    }
}
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

    let bad = sig(
        json!({"kind": "function", "name": "firstBadVersion", "special": "bad_version",
        "params": [{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}],
        "returns": "int"}),
    );
    let code = "
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut lo, mut hi) = (1, n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.isBadVersion(mid) { hi = mid; } else { lo = mid + 1; }
        }
        lo
    }
}
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

    let values = sig(json!({"kind": "function", "name": "f",
        "params": [{"name": "grid", "type": "char[][]"}, {"name": "big", "type": "long"}, {"name": "x", "type": "double"}, {"name": "b", "type": "bool"}],
        "returns": "string[]"}));
    let code = "impl Solution { pub fn f(grid: Vec<Vec<char>>, big: i64, x: f64, b: bool) -> Vec<String> { vec![grid[0][1].to_string(), (big + 1).to_string(), format!(\"{}\", x * 2.0), b.to_string()] } }";
    let r = run(
        &values,
        code,
        &[(
            json!([[["a", "z"]], 9_000_000_000i64, 1.5, true]),
            json!(["z", "9000000001", "3", "true"]),
        )],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn starters_compile_once_bodies_are_filled() {
    let sigs = [
        two_sum(),
        sig(
            json!({"kind": "design", "class": "MinStack", "constructor": [],
            "methods": [{"name": "push", "params": [{"name": "val", "type": "int"}], "returns": "void"},
                        {"name": "getMin", "returns": "int"}]}),
        ),
        sig(json!({"kind": "codec", "class": "Codec",
            "encode": {"name": "serialize", "params": [{"name": "root", "type": "TreeNode"}], "returns": "string"},
            "decode": {"name": "deserialize", "params": [{"name": "data", "type": "string"}], "returns": "TreeNode"}})),
    ];
    let inputs = [
        json!([[1, 2], 3]),
        json!([["MinStack", "push"], [[], [1]]]),
        json!([[1]]),
    ];
    for (s, input) in sigs.iter().zip(inputs) {
        let starter = Language::Rust.driver().starter_code(s);
        let filled = starter.replace("{\n        \n    }", "{\n        todo!()\n    }");
        let r = run(s, &filled, &[(input, json!(null))]);
        assert_eq!(r.verdict, Verdict::RuntimeError, "{filled}\n{:?}", r.error);
        assert!(r.error.unwrap().contains("not yet implemented"));
    }
}
