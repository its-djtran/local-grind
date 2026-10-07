//! Compiles and runs real C++ solutions through the generated harness and the runner.

use localgrind_core::lang::{Language, Toolchains};
use localgrind_core::runner::{execute, RunOptions, RunReport, TestCase, Verdict};
use localgrind_core::types::{Comparator, Signature};
use serde_json::{json, Value};

fn sig(v: Value) -> Signature {
    serde_json::from_value(v).unwrap()
}

fn run_with(sig: &Signature, code: &str, cases: &[(Value, Value)], cmp: Comparator) -> RunReport {
    let program = Language::Cpp
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

#[test]
fn accepted_with_stdout() {
    let code = "
class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        cout << \"target \" << target << endl;
        unordered_map<int, int> seen;
        for (int i = 0; i < (int)nums.size(); i++) {
            auto it = seen.find(target - nums[i]);
            if (it != seen.end()) return {it->second, i};
            seen[nums[i]] = i;
        }
        return {};
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
    assert_eq!(r.cases[1].stdout, "target 6\n");
}

#[test]
fn compile_errors_point_at_user_lines() {
    let code = "class Solution {\npublic:\n    vector<int> twoSum(vector<int>& nums, int target) {\n        return nope;\n    }\n};\n";
    let r = run(&two_sum(), code, &[(json!([[1], 1]), json!([]))]);
    assert_eq!(r.verdict, Verdict::CompileError);
    let err = r.error.unwrap();
    assert!(err.contains("solution.cpp:4"), "{err}");
}

#[test]
fn crashes_and_exceptions_are_runtime_errors() {
    // An opaque null pointer really faults; a provable one is compiled into a trap.
    let segv = "class Solution { public: vector<int> twoSum(vector<int>& nums, int target) { volatile int* p = target > 100 ? &nums[0] : nullptr; *p = 1; return {}; } };";
    let r = run(&two_sum(), segv, &[(json!([[1], 1]), json!([]))]);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    let err = r.error.unwrap();
    assert!(
        err.contains("segmentation fault") || err.contains("bus error"),
        "{err}"
    );
    let ub = "class Solution { public: vector<int> twoSum(vector<int>& nums, int target) { int* p = nullptr; *p = 1; return {*p}; } };";
    let r = run(&two_sum(), ub, &[(json!([[1], 1]), json!([]))]);
    assert_eq!(r.verdict, Verdict::RuntimeError);

    let throws = "class Solution { public: vector<int> twoSum(vector<int>& nums, int target) { return {nums.at(5)}; } };";
    let r = run(&two_sum(), throws, &[(json!([[1], 1]), json!([]))]);
    assert_eq!(r.verdict, Verdict::RuntimeError);
    assert!(r.error.unwrap().contains("what():"));

    let unknown = "class Solution { public: vector<int> twoSum(vector<int>& nums, int target) { throw 42; } };";
    let r = run(&two_sum(), unknown, &[(json!([[1], 1]), json!([]))]);
    assert!(r.error.unwrap().contains("unknown exception"));
}

#[test]
fn value_types_roundtrip() {
    let s = sig(json!({"kind": "function", "name": "f",
        "params": [{"name": "words", "type": "string[]"}, {"name": "grid", "type": "char[][]"},
                   {"name": "big", "type": "long"}, {"name": "flag", "type": "bool"}, {"name": "x", "type": "double"}],
        "returns": "string[]"}));
    let code = r#"
class Solution {
public:
    vector<string> f(vector<string>& words, vector<vector<char>>& grid, long long big, bool flag, double x) {
        vector<string> out = words;
        out.push_back(string(1, grid[0][1]));
        out.push_back(to_string(big + 1));
        out.push_back(flag ? "yes" : "no");
        out.push_back(to_string((int)(x * 2)));
        out.push_back("quote\" back\\ tab\t é");
        return out;
    }
};
"#;
    let r = run(
        &s,
        code,
        &[(
            json!([["a", "ü\n"], [["x", "y"]], 9_000_000_000i64, true, 1.5]),
            json!([
                "a",
                "ü\n",
                "y",
                "9000000001",
                "yes",
                "3",
                "quote\" back\\ tab\t é"
            ]),
        )],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let b = sig(json!({"kind": "function", "name": "g",
        "params": [{"name": "n", "type": "int"}], "returns": "bool[]"}));
    let r = run(
        &b,
        "class Solution { public: vector<bool> g(int n) { return vector<bool>(n, true); } };",
        &[(json!([2]), json!([true, true]))],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let d = sig(json!({"kind": "function", "name": "h",
        "params": [{"name": "v", "type": "int[]"}], "returns": "double"}));
    let r = run_with(
        &d,
        "class Solution { public: double h(vector<int>& v) { return accumulate(v.begin(), v.end(), 0.0) / v.size(); } };",
        &[(json!([[1, 2]]), json!(1.5))],
        Comparator::Float { tolerance: 1e-6 },
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn lists_trees_and_deep_recursion() {
    let reverse = sig(json!({"kind": "function", "name": "reverseList",
        "params": [{"name": "head", "type": "ListNode"}], "returns": "ListNode"}));
    let code = "class Solution { public: ListNode* reverseList(ListNode* head) { ListNode* p = nullptr; while (head) { ListNode* n = head->next; head->next = p; p = head; head = n; } return p; } };";
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
    let code = "class Solution { public: TreeNode* invertTree(TreeNode* r) { if (r) { TreeNode* l = invertTree(r->left); r->left = invertTree(r->right); r->right = l; } return r; } };";
    let mut spine = vec![json!(0)];
    let mut mirrored = vec![json!(0)];
    for i in 1..100_000 {
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
        "class Solution { public: ListNode* f(ListNode* h) { h->next = h; return h; } };",
        &[(json!([[1, 2]]), json!([]))],
    );
    assert!(r.error.unwrap().contains("cycle"));

    let merge = sig(json!({"kind": "function", "name": "mergeKLists",
        "params": [{"name": "lists", "type": "ListNode[]"}], "returns": "ListNode"}));
    let code = "class Solution { public: ListNode* mergeKLists(vector<ListNode*>& lists) { vector<int> v; for (auto l : lists) for (; l; l = l->next) v.push_back(l->val); sort(v.begin(), v.end()); ListNode d; ListNode* c = &d; for (int x : v) c = c->next = new ListNode(x); return d.next; } };";
    let r = run(
        &merge,
        code,
        &[(
            json!([[[1, 4, 5], [1, 3, 4], [2, 6]]]),
            json!([1, 1, 2, 3, 4, 4, 5, 6]),
        )],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn graphs_with_deep_copy() {
    let s = sig(
        json!({"kind": "function", "name": "cloneGraph", "special": "deep_copy",
        "params": [{"name": "node", "type": "Node"}], "returns": "Node"}),
    );
    let good = "
class Solution {
    unordered_map<Node*, Node*> copies;
public:
    Node* cloneGraph(Node* node) {
        if (!node) return nullptr;
        if (copies.count(node)) return copies[node];
        Node* c = new Node(node->val);
        copies[node] = c;
        for (Node* n : node->neighbors) c->neighbors.push_back(cloneGraph(n));
        return c;
    }
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
        "class Solution { public: Node* cloneGraph(Node* n) { return n; } };",
        &[(json!([adj.clone()]), adj)],
    );
    assert!(r.error.unwrap().contains("deep copy"));
}

#[test]
fn in_place_design_and_codec() {
    let sort = sig(json!({"kind": "function", "name": "sortColors",
        "params": [{"name": "nums", "type": "int[]"}], "returns": "void", "output_param": "nums"}));
    let r = run(&sort, "class Solution { public: void sortColors(vector<int>& nums) { sort(nums.begin(), nums.end()); } };", &[(json!([[2, 0, 1]]), json!([0, 1, 2]))]);
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let lru = sig(json!({"kind": "design", "class": "LRUCache",
        "constructor": [{"name": "capacity", "type": "int"}],
        "methods": [{"name": "get", "params": [{"name": "key", "type": "int"}], "returns": "int"},
                    {"name": "put", "params": [{"name": "key", "type": "int"}, {"name": "value", "type": "int"}], "returns": "void"}]}));
    let code = "
class LRUCache {
    int cap;
    list<pair<int, int>> items;
    unordered_map<int, list<pair<int, int>>::iterator> pos;
public:
    LRUCache(int capacity) : cap(capacity) {}
    int get(int key) {
        auto it = pos.find(key);
        if (it == pos.end()) return -1;
        items.splice(items.begin(), items, it->second);
        return it->second->second;
    }
    void put(int key, int value) {
        if (get(key) != -1) { items.front().second = value; return; }
        items.emplace_front(key, value);
        pos[key] = items.begin();
        if ((int)items.size() > cap) { pos.erase(items.back().first); items.pop_back(); }
    }
};
";
    let ops = json!(["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]);
    let args = json!([[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]);
    let r = run(
        &lru,
        code,
        &[(
            json!([ops, args]),
            json!([null, null, null, 1, null, -1, null, -1, 3, 4]),
        )],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");

    let unknown = run(
        &lru,
        code,
        &[(json!([["LRUCache", "nope"], [[1], []]]), json!([]))],
    );
    assert!(unknown.error.unwrap().contains("unknown operation nope"));

    let codec = sig(json!({"kind": "codec", "class": "Codec",
        "encode": {"name": "serialize", "params": [{"name": "root", "type": "TreeNode"}], "returns": "string"},
        "decode": {"name": "deserialize", "params": [{"name": "data", "type": "string"}], "returns": "TreeNode"}}));
    let code = "
class Codec {
public:
    string serialize(TreeNode* root) {
        if (!root) return \"#\";
        return to_string(root->val) + \",\" + serialize(root->left) + \",\" + serialize(root->right);
    }
    TreeNode* deserialize(string data) {
        stringstream ss(data);
        return build(ss);
    }
private:
    TreeNode* build(stringstream& ss) {
        string tok;
        getline(ss, tok, ',');
        if (tok == \"#\") return nullptr;
        TreeNode* n = new TreeNode(stoi(tok));
        n->left = build(ss);
        n->right = build(ss);
        return n;
    }
};
";
    let tree = json!([1, 2, 3, null, null, 4, 5]);
    let r = run(
        &codec,
        code,
        &[(json!([tree.clone()]), tree), (json!([[]]), json!([]))],
    );
    assert_eq!(r.verdict, Verdict::Accepted, "{r:#?}");
}

#[test]
fn specials_node_refs_cycles_and_bad_version() {
    let lca = sig(json!({"kind": "function", "name": "lowestCommonAncestor",
        "params": [{"name": "root", "type": "TreeNode"},
                   {"name": "p", "type": "TreeNode", "ref_of": "root"},
                   {"name": "q", "type": "TreeNode", "ref_of": "root"}],
        "returns": "TreeNode", "output_mode": "node_val"}));
    let code = "class Solution { public: TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) { while (root) { if (p->val < root->val && q->val < root->val) root = root->left; else if (p->val > root->val && q->val > root->val) root = root->right; else return root; } return nullptr; } };";
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
    let code = "class Solution { public: bool hasCycle(ListNode* head) { auto s = head, f = head; while (f && f->next) { s = s->next; f = f->next->next; if (s == f) return true; } return false; } };";
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
    let code = "class Solution { public: int firstBadVersion(int n) { int lo = 1, hi = n; while (lo < hi) { int mid = lo + (hi - lo) / 2; if (isBadVersion(mid)) hi = mid; else lo = mid + 1; } return lo; } };";
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
