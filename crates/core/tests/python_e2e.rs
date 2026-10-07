//! Runs real Python solutions through the generated harness and the runner.

use localgrind_core::lang::{Language, Toolchains};
use localgrind_core::runner::{execute, RunOptions, RunReport, TestCase, Verdict};
use localgrind_core::types::{Comparator, Signature};
use serde_json::{json, Value};

fn sig(v: Value) -> Signature {
    serde_json::from_value(v).unwrap()
}

fn run_with(sig: &Signature, code: &str, cases: &[(Value, Value)], cmp: Comparator) -> RunReport {
    let program = Language::Python
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
    execute(&program, &cases, &cmp, &opts, &mut |_, _| {}).unwrap()
}

fn run(sig: &Signature, code: &str, cases: &[(Value, Value)]) -> RunReport {
    run_with(sig, code, cases, Comparator::Exact)
}

fn two_sum() -> Signature {
    sig(json!({"kind": "function", "name": "twoSum",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
        "returns": "int[]"}))
}

const TWO_SUM: &str = "
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i, n in enumerate(nums):
            if target - n in seen:
                return [seen[target - n], i]
            seen[n] = i
";

#[test]
fn accepted_two_sum() {
    let r = run(
        &two_sum(),
        TWO_SUM,
        &[
            (json!([[2, 7, 11, 15], 9]), json!([0, 1])),
            (json!([[3, 2, 4], 6]), json!([1, 2])),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
    assert_eq!(r.passed, 2);
    assert!(r.memory_kb > 1000);
}

#[test]
fn wrong_answer_and_stdout() {
    let code = "class Solution:\n    def twoSum(self, nums, target):\n        print('dbg', target)\n        return [1, 0]\n";
    let r = run(&two_sum(), code, &[(json!([[2, 7], 9]), json!([0, 1]))]);
    assert_eq!(r.verdict, Verdict::WrongAnswer);
    let f = r.failure.unwrap();
    assert_eq!(f.output, Some(json!([1, 0])));
    assert_eq!(f.stdout, "dbg 9\n");
}

#[test]
fn syntax_error_is_compile_error() {
    let r = run(
        &two_sum(),
        "class Solution:\n    def twoSum(self, nums, target):\n",
        &[(json!([[1], 1]), json!([]))],
    );
    assert_eq!(r.verdict, Verdict::CompileError);
    assert!(r.error.unwrap().contains("Line 2"), "expected line number");
}

#[test]
fn runtime_error_has_user_traceback() {
    let code = "class Solution:\n    def twoSum(self, nums, target):\n        return nums[100]\n";
    let r = run(&two_sum(), code, &[(json!([[1], 1]), json!([]))]);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let err = r.error.unwrap();
    assert!(err.contains("IndexError"), "{err}");
    assert!(err.contains("solution.py\", line 3"), "{err}");
    assert!(!err.contains("lg_harness"), "{err}");
}

#[test]
fn import_time_error_is_runtime_error() {
    let r = run(
        &two_sum(),
        "raise ValueError('top')\n",
        &[(json!([[1], 1]), json!([]))],
    );
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(r.error.unwrap().contains("ValueError: top"));
}

#[test]
fn infinite_loop_times_out() {
    let code = "class Solution:\n    def twoSum(self, nums, target):\n        while True: pass\n";
    let program = Language::Python
        .driver()
        .program(&two_sum(), code, &Toolchains::detect());
    let mut opts = RunOptions::default();
    opts.limits.wall_limit_ms = 1500;
    let cases = [TestCase {
        args: json!([[1], 1]),
        expected: json!([]),
    }];
    let r = execute(&program, &cases, &Comparator::Exact, &opts, &mut |_, _| {}).unwrap();
    assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
}

#[test]
fn linked_lists_roundtrip() {
    let s = sig(json!({"kind": "function", "name": "reverseList",
        "params": [{"name": "head", "type": "ListNode"}], "returns": "ListNode"}));
    let code = "
class Solution:
    def reverseList(self, head):
        prev = None
        while head:
            head.next, prev, head = prev, head, head.next
        return prev
";
    let r = run(
        &s,
        code,
        &[
            (json!([[1, 2, 3]]), json!([3, 2, 1])),
            (json!([[]]), json!([])),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn cyclic_list_output_is_runtime_error() {
    let s = sig(json!({"kind": "function", "name": "f",
        "params": [{"name": "head", "type": "ListNode"}], "returns": "ListNode"}));
    let code =
        "class Solution:\n    def f(self, head):\n        head.next = head\n        return head\n";
    let r = run(&s, code, &[(json!([[1, 2]]), json!([]))]);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(r.error.unwrap().contains("cycle"));
}

#[test]
fn trees_roundtrip_and_deep_recursion() {
    let s = sig(json!({"kind": "function", "name": "invertTree",
        "params": [{"name": "root", "type": "TreeNode"}], "returns": "TreeNode"}));
    let code = "
class Solution:
    def invertTree(self, root):
        if root:
            root.left, root.right = self.invertTree(root.right), self.invertTree(root.left)
        return root
";
    // A 20k-deep left spine exercises the raised recursion limit and stack size.
    let mut spine = vec![json!(0)];
    for i in 1..20_000 {
        spine.push(json!(i));
        spine.push(Value::Null);
    }
    let mut mirrored = vec![json!(0)];
    for i in 1..20_000 {
        mirrored.push(Value::Null);
        mirrored.push(json!(i));
    }
    let r = run(
        &s,
        code,
        &[
            (json!([[4, 2, 7, 1, 3, 6, 9]]), json!([4, 7, 2, 9, 6, 3, 1])),
            (json!([[]]), json!([])),
            (json!([spine]), Value::Array(mirrored)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{:?}", r.error);
}

#[test]
fn graph_nodes_roundtrip() {
    let s = sig(json!({"kind": "function", "name": "cloneGraph",
        "params": [{"name": "node", "type": "Node"}], "returns": "Node"}));
    let code = "
class Solution:
    def cloneGraph(self, node):
        if not node: return None
        copies = {}
        def dfs(n):
            if n in copies: return copies[n]
            c = Node(n.val)
            copies[n] = c
            c.neighbors = [dfs(x) for x in n.neighbors]
            return c
        return dfs(node)
";
    let adj = json!([[2, 4], [1, 3], [2, 4], [1, 3]]);
    let r = run(
        &s,
        code,
        &[(json!([adj.clone()]), adj), (json!([[]]), json!([]))],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn in_place_output_param() {
    let s = sig(json!({"kind": "function", "name": "sortColors",
        "params": [{"name": "nums", "type": "int[]"}], "returns": "void", "output_param": "nums"}));
    let r = run(
        &s,
        "class Solution:\n    def sortColors(self, nums):\n        nums.sort()\n",
        &[(json!([[2, 0, 1]]), json!([0, 1, 2]))],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn design_problem() {
    let s = sig(
        json!({"kind": "design", "class": "MinStack", "constructor": [],
        "methods": [
            {"name": "push", "params": [{"name": "val", "type": "int"}], "returns": "void"},
            {"name": "pop", "returns": "void"},
            {"name": "top", "returns": "int"},
            {"name": "getMin", "returns": "int"}]}),
    );
    let code = "
class MinStack:
    def __init__(self):
        self.s = []
    def push(self, val):
        self.s.append((val, min(val, self.s[-1][1] if self.s else val)))
    def pop(self):
        self.s.pop()
    def top(self):
        return self.s[-1][0]
    def getMin(self):
        return self.s[-1][1]
";
    let ops = json!(["MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin"]);
    let args = json!([[], [-2], [0], [-3], [], [], [], []]);
    let r = run(
        &s,
        code,
        &[(
            json!([ops, args]),
            json!([null, null, null, null, -3, null, 0, -2]),
        )],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn doubles_and_float_comparator() {
    let s = sig(json!({"kind": "function", "name": "avg",
        "params": [{"name": "nums", "type": "int[]"}, {"name": "scale", "type": "double"}],
        "returns": "double"}));
    let code = "class Solution:\n    def avg(self, nums, scale):\n        return sum(nums) / len(nums) * scale\n";
    let r = run_with(
        &s,
        code,
        &[
            (json!([[1, 2], 1]), json!(1.5)),
            (json!([[1, 2, 2], 3]), json!(5.0)),
        ],
        Comparator::Float { tolerance: 1e-5 },
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn nested_special_types() {
    let s = sig(json!({"kind": "function", "name": "mergeKLists",
        "params": [{"name": "lists", "type": "ListNode[]"}], "returns": "ListNode"}));
    let code = "
class Solution:
    def mergeKLists(self, lists):
        vals = []
        for l in lists:
            while l:
                vals.append(l.val); l = l.next
        return make_list(sorted(vals))
";
    let r = run(
        &s,
        code,
        &[(
            json!([[[1, 4, 5], [1, 3, 4], [2, 6]]]),
            json!([1, 1, 2, 3, 4, 4, 5, 6]),
        )],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn lowest_common_ancestor_node_refs() {
    let s = sig(json!({"kind": "function", "name": "lowestCommonAncestor",
        "params": [{"name": "root", "type": "TreeNode"},
                   {"name": "p", "type": "TreeNode", "ref_of": "root"},
                   {"name": "q", "type": "TreeNode", "ref_of": "root"}],
        "returns": "TreeNode", "output_mode": "node_val"}));
    let code = "
class Solution:
    def lowestCommonAncestor(self, root, p, q):
        assert isinstance(p, TreeNode) and isinstance(q, TreeNode)
        while root:
            if p.val < root.val and q.val < root.val: root = root.left
            elif p.val > root.val and q.val > root.val: root = root.right
            else: return root
";
    let tree = json!([6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]);
    let r = run(
        &s,
        code,
        &[
            (json!([tree.clone(), 2, 8]), json!(6)),
            (json!([tree, 2, 4]), json!(2)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn linked_list_cycle_hidden_pos() {
    let s = sig(
        json!({"kind": "function", "name": "hasCycle", "special": "list_cycle",
        "params": [{"name": "head", "type": "ListNode"}, {"name": "pos", "type": "int", "hidden": true}],
        "returns": "bool"}),
    );
    let code = "
class Solution:
    def hasCycle(self, head):
        slow = fast = head
        while fast and fast.next:
            slow, fast = slow.next, fast.next.next
            if slow is fast: return True
        return False
";
    let r = run(
        &s,
        code,
        &[
            (json!([[3, 2, 0, -4], 1]), json!(true)),
            (json!([[1], -1]), json!(false)),
            (json!([[], -1]), json!(false)),
            (json!([[1, 2], 0]), json!(true)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn first_bad_version_api() {
    let s = sig(
        json!({"kind": "function", "name": "firstBadVersion", "special": "bad_version",
        "params": [{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}],
        "returns": "int"}),
    );
    let code = "
class Solution:
    def firstBadVersion(self, n):
        lo, hi = 1, n
        while lo < hi:
            mid = (lo + hi) // 2
            if isBadVersion(mid): hi = mid
            else: lo = mid + 1
        return lo
";
    let r = run(
        &s,
        code,
        &[
            (json!([5, 4]), json!(4)),
            (json!([2147483647, 2]), json!(2)),
        ],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn deep_copy_is_enforced() {
    let s = sig(
        json!({"kind": "function", "name": "cloneGraph", "special": "deep_copy",
        "params": [{"name": "node", "type": "Node"}], "returns": "Node"}),
    );
    let r = run(
        &s,
        "class Solution:\n    def cloneGraph(self, node):\n        return node\n",
        &[(json!([[[2], [1]]]), json!([[2], [1]]))],
    );
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(r.error.unwrap().contains("deep copy"));
}

#[test]
fn codec_round_trips() {
    let s = sig(json!({"kind": "codec", "class": "Codec",
        "encode": {"name": "encode", "params": [{"name": "strs", "type": "string[]"}], "returns": "string"},
        "decode": {"name": "decode", "params": [{"name": "s", "type": "string"}], "returns": "string[]"}}));
    let good = "
class Codec:
    def encode(self, strs):
        return ''.join('%d#%s' % (len(x), x) for x in strs)
    def decode(self, s):
        out, i = [], 0
        while i < len(s):
            j = s.index('#', i)
            n = int(s[i:j])
            out.append(s[j + 1:j + 1 + n])
            i = j + 1 + n
        return out
";
    let cases = [(json!([["lint", "co#de", ""]]), json!(["lint", "co#de", ""]))];
    assert_eq!(run(&s, good, &cases).verdict, Verdict::Accepted);
    // State shared through the class cannot survive: encode and decode use separate instances.
    let cheat = "
class Codec:
    def encode(self, strs):
        self.saved = strs
        return 'x'
    def decode(self, s):
        return self.saved
";
    assert_eq!(run(&s, cheat, &cases).verdict, Verdict::RuntimeError);
    let not_str = "class Codec:\n    def encode(self, strs): return strs\n    def decode(self, s): return s\n";
    let r = run(&s, not_str, &cases);
    assert!(r.error.unwrap().contains("must return a str"));
}
