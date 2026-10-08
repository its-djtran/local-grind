//! C++ driver. User code is compiled together with a static prelude/driver and a generated
//! `lg_case.inc` that decodes one test into native types and calls the solution.

use super::{Driver, Language, Program, Toolchains};
use crate::types::{Method, OutputMode, Param, Signature, Special, ValueType};

const PRELUDE: &str = include_str!("../../drivers/cpp/lg_prelude.hpp");
const DRIVER: &str = include_str!("../../drivers/cpp/lg_driver.hpp");

pub struct CppDriver;

fn cpp_type(t: &ValueType) -> String {
    match t {
        ValueType::Int => "int".into(),
        ValueType::Long => "long long".into(),
        ValueType::Double => "double".into(),
        ValueType::Bool => "bool".into(),
        ValueType::String => "string".into(),
        ValueType::Char => "char".into(),
        ValueType::ListNode => "ListNode*".into(),
        ValueType::TreeNode => "TreeNode*".into(),
        ValueType::GraphNode => "Node*".into(),
        ValueType::Void => "void".into(),
        ValueType::List(inner) => format!("vector<{}>", cpp_type(inner)),
    }
}

/// Parameter declaration as LeetCode writes it: containers by reference, the rest by value.
fn param_decl(p: &Param) -> String {
    match p.ty {
        ValueType::List(_) => format!("{}& {}", cpp_type(&p.ty), p.name),
        _ => format!("{} {}", cpp_type(&p.ty), p.name),
    }
}

fn params(ps: &[Param]) -> String {
    ps.iter()
        .filter(|p| !p.hidden)
        .map(param_decl)
        .collect::<Vec<_>>()
        .join(", ")
}

fn helper_comments(sig: &Signature) -> String {
    let mut out = String::new();
    if sig.uses(&ValueType::ListNode) {
        out.push_str(
            "/**\n * Definition for singly-linked list.\n * struct ListNode {\n *     int val;\n\
             \x20*     ListNode *next;\n *     ListNode() : val(0), next(nullptr) {}\n\
             \x20*     ListNode(int x) : val(x), next(nullptr) {}\n\
             \x20*     ListNode(int x, ListNode *next) : val(x), next(next) {}\n * };\n */\n",
        );
    }
    if sig.uses(&ValueType::TreeNode) {
        out.push_str(
            "/**\n * Definition for a binary tree node.\n * struct TreeNode {\n *     int val;\n\
             \x20*     TreeNode *left;\n *     TreeNode *right;\n\
             \x20*     TreeNode() : val(0), left(nullptr), right(nullptr) {}\n\
             \x20*     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}\n\
             \x20*     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}\n\
             \x20* };\n */\n",
        );
    }
    if sig.uses(&ValueType::GraphNode) {
        out.push_str(
            "/*\n// Definition for a Node.\nclass Node {\npublic:\n    int val;\n    vector<Node*> neighbors;\n\
             \x20   Node() {\n        val = 0;\n        neighbors = vector<Node*>();\n    }\n\
             \x20   Node(int _val) {\n        val = _val;\n        neighbors = vector<Node*>();\n    }\n\
             \x20   Node(int _val, vector<Node*> _neighbors) {\n        val = _val;\n        neighbors = _neighbors;\n    }\n};\n*/\n",
        );
    }
    out
}

fn method_stub(m: &Method) -> String {
    format!(
        "    {} {}({}) {{\n        \n    }}\n",
        cpp_type(&m.returns),
        m.name,
        params(&m.params)
    )
}

/// `auto aN = lg::Dec<T>::get(src);` for each parameter, honouring node references.
fn decode_params(ps: &[Param], src: &str, prefix: &str) -> String {
    let mut out = String::new();
    for (i, p) in ps.iter().enumerate() {
        if let Some(target) = &p.ref_of {
            let t = ps.iter().position(|q| &q.name == target).unwrap_or(0);
            out.push_str(&format!(
                "    auto {prefix}{i} = lg::find({prefix}{t}, {src}[{i}]);\n"
            ));
        } else {
            out.push_str(&format!(
                "    auto {prefix}{i} = lg::Dec<{}>::get({src}[{i}]);\n",
                cpp_type(&p.ty)
            ));
        }
    }
    out
}

fn call_args(ps: &[Param], prefix: &str) -> String {
    ps.iter()
        .enumerate()
        .filter(|(_, p)| !p.hidden)
        .map(|(i, _)| format!("{prefix}{i}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Declarations user code may rely on before it is compiled.
fn api_header(sig: &Signature) -> String {
    match sig {
        Signature::Function {
            special: Some(Special::BadVersion),
            ..
        } => "bool isBadVersion(int version);\n".into(),
        _ => String::new(),
    }
}

/// Generated `lg_case`: decodes one test, calls the solution, encodes the result.
pub fn case_source(sig: &Signature) -> String {
    let mut out = String::new();
    match sig {
        Signature::Function {
            name,
            params: ps,
            returns,
            output_param,
            special,
            output_mode,
        } => {
            let hidden = ps.iter().position(|p| p.hidden);
            if *special == Some(Special::BadVersion) {
                out.push_str(
                    "static long long lg_bad = 0;\nbool isBadVersion(int version) { return version >= lg_bad; }\n\n",
                );
            }
            out.push_str("long long lg_case(const lg::J& args, std::string& out) {\n");
            out.push_str(&decode_params(ps, "args.items", "a"));
            match (special, hidden) {
                (Some(Special::ListCycle), Some(h)) => {
                    let list = ps
                        .iter()
                        .position(|p| p.ty == ValueType::ListNode)
                        .unwrap_or(0);
                    out.push_str(&format!("    lg::link_cycle(a{list}, a{h});\n"));
                }
                (Some(Special::BadVersion), Some(h)) => {
                    out.push_str(&format!("    lg_bad = a{h};\n"));
                }
                _ => {}
            }
            let call = format!("sol->{name}({})", call_args(ps, "a"));
            // Owned per case (freed after the result is encoded, as LeetCode's stack-allocated
            // Solution is) so member state such as memo tables does not pile up across cases.
            // Input structures are not freed: solutions may delete nodes they remove.
            out.push_str(
                "    std::unique_ptr<Solution> sol(new Solution());\n    auto t0 = lg::clock::now();\n",
            );
            if *returns == ValueType::Void {
                out.push_str(&format!("    {call};\n"));
            } else {
                out.push_str(&format!("    auto res = {call};\n"));
            }
            out.push_str("    auto t1 = lg::clock::now();\n");
            if *special == Some(Special::DeepCopy) {
                out.push_str("    lg::check_deep_copy(a0, res);\n");
            }
            if let Some(param) = output_param {
                let idx = ps.iter().position(|p| &p.name == param).unwrap_or(0);
                out.push_str(&format!("    lg::enc(out, a{idx});\n"));
            } else if *output_mode == Some(OutputMode::NodeVal) {
                out.push_str("    if (res) lg::enc(out, res->val); else out += \"null\";\n");
            } else {
                out.push_str("    lg::enc(out, res);\n");
            }
            out.push_str("    return lg::ns(t0, t1);\n}\n");
        }
        Signature::Design {
            class,
            constructor,
            methods,
        } => {
            out.push_str(&format!(
                "long long lg_case(const lg::J& args, std::string& out) {{\n\
                 \x20   const auto& ops = args.items[0].items;\n\
                 \x20   const auto& opargs = args.items[1].items;\n\
                 \x20   long long total = 0;\n    std::unique_ptr<{class}> obj;\n    out += '[';\n\
                 \x20   for (size_t k = 0; k < ops.size(); k++) {{\n\
                 \x20       if (k) out += ',';\n\
                 \x20       const std::string& op = ops[k].text;\n\
                 \x20       const auto& a = opargs[k].items;\n\
                 \x20       if (k == 0) {{\n"
            ));
            out.push_str(&indent(&decode_params(constructor, "a", "c"), 8));
            out.push_str(&format!(
                "            auto t0 = lg::clock::now();\n            obj.reset(new {class}({}));\n\
                 \x20           total += lg::ns(t0, lg::clock::now());\n            out += \"null\";\n        }}",
                call_args(constructor, "c")
            ));
            for m in methods {
                out.push_str(&format!(" else if (op == \"{}\") {{\n", m.name));
                out.push_str(&indent(&decode_params(&m.params, "a", "p"), 8));
                let call = format!("obj->{}({})", m.name, call_args(&m.params, "p"));
                out.push_str("            auto t0 = lg::clock::now();\n");
                if m.returns == ValueType::Void {
                    out.push_str(&format!(
                        "            {call};\n            total += lg::ns(t0, lg::clock::now());\n            out += \"null\";\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "            auto r = {call};\n            total += lg::ns(t0, lg::clock::now());\n            lg::enc(out, r);\n"
                    ));
                }
                out.push_str("        }");
            }
            out.push_str(
                " else {\n            throw std::runtime_error(\"unknown operation \" + op);\n        }\n    }\n\
                 \x20   out += ']';\n    return total;\n}\n",
            );
        }
        Signature::Codec {
            class,
            encode,
            decode,
        } => {
            out.push_str(&format!(
                "long long lg_case(const lg::J& args, std::string& out) {{\n\
                 \x20   auto v = lg::Dec<{}>::get(args.items[0]);\n\
                 \x20   std::unique_ptr<{class}> ser(new {class}());\n\
                 \x20   std::unique_ptr<{class}> deser(new {class}());\n\
                 \x20   auto t0 = lg::clock::now();\n    auto data = ser->{}(v);\n\
                 \x20   auto res = deser->{}(data);\n    auto t1 = lg::clock::now();\n\
                 \x20   lg::enc(out, res);\n    return lg::ns(t0, t1);\n}}\n",
                cpp_type(&encode.params[0].ty),
                encode.name,
                decode.name
            ));
        }
    }
    out
}

fn indent(s: &str, by: usize) -> String {
    let pad = " ".repeat(by);
    s.lines().map(|l| format!("{pad}{l}\n")).collect()
}

impl Driver for CppDriver {
    fn language(&self) -> Language {
        Language::Cpp
    }

    fn starter_code(&self, sig: &Signature) -> String {
        let mut out = helper_comments(sig);
        match sig {
            Signature::Function {
                name,
                params: ps,
                returns,
                special,
                ..
            } => {
                if *special == Some(Special::BadVersion) {
                    out.push_str("// The API isBadVersion is defined for you.\n// bool isBadVersion(int version);\n\n");
                }
                out.push_str(&format!(
                    "class Solution {{\npublic:\n    {} {name}({}) {{\n        \n    }}\n}};\n",
                    cpp_type(returns),
                    params(ps)
                ));
            }
            Signature::Design {
                class,
                constructor,
                methods,
            } => {
                out.push_str(&format!(
                    "class {class} {{\npublic:\n    {class}({}) {{\n        \n    }}\n",
                    params(constructor)
                ));
                for m in methods {
                    out.push_str("    \n");
                    out.push_str(&method_stub(m));
                }
                let ctor: Vec<&str> = constructor.iter().map(|p| p.name.as_str()).collect();
                out.push_str(&format!(
                    "}};\n\n/**\n * Your {class} object will be instantiated and called as such:\n * {class}* obj = new {class}({});\n",
                    ctor.join(", ")
                ));
                for (i, m) in methods.iter().enumerate() {
                    let args: Vec<&str> = m.params.iter().map(|p| p.name.as_str()).collect();
                    let call = format!("obj->{}({});", m.name, args.join(","));
                    if m.returns == ValueType::Void {
                        out.push_str(&format!(" * {call}\n"));
                    } else {
                        out.push_str(&format!(
                            " * {} param_{} = {call}\n",
                            cpp_type(&m.returns),
                            i + 1
                        ));
                    }
                }
                out.push_str(" */\n");
            }
            Signature::Codec {
                class,
                encode,
                decode,
            } => {
                out.push_str(&format!("class {class} {{\npublic:\n"));
                out.push_str(&method_stub(encode));
                out.push('\n');
                out.push_str(&method_stub(decode));
                out.push_str(&format!(
                    "}};\n\n// Your {class} object will be instantiated and called as such:\n// {class} ser, deser;\n// {} ans = deser.{}(ser.{}({}));\n",
                    cpp_type(&decode.returns),
                    decode.name,
                    encode.name,
                    encode.params[0].name
                ));
            }
        }
        out
    }

    fn program(&self, sig: &Signature, user_code: &str, tools: &Toolchains) -> Program {
        let source = format!(
            "#include \"lg_prelude.hpp\"\n#include \"lg_api.hpp\"\n#line 1 \"solution.cpp\"\n{user_code}\n\
             #include \"lg_driver.hpp\"\n#include \"lg_case.inc\"\n"
        );
        Program {
            files: vec![
                ("solution.cpp".into(), source),
                ("lg_prelude.hpp".into(), PRELUDE.into()),
                ("lg_api.hpp".into(), api_header(sig)),
                ("lg_driver.hpp".into(), DRIVER.into()),
                ("lg_case.inc".into(), case_source(sig)),
            ],
            compile: Some(vec![
                tools.path(Language::Cpp).to_string_lossy().into_owned(),
                "-std=c++20".into(),
                "-O2".into(),
                "-w".into(),
                "-o".into(),
                "prog".into(),
                "solution.cpp".into(),
            ]),
            artifacts: vec!["prog".into()],
            run: vec!["./prog".into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sig(v: serde_json::Value) -> Signature {
        serde_json::from_value(v).unwrap()
    }

    #[test]
    fn function_starter() {
        let s = sig(json!({"kind": "function", "name": "twoSum",
            "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
            "returns": "int[]"}));
        assert_eq!(
            CppDriver.starter_code(&s),
            "class Solution {\npublic:\n    vector<int> twoSum(vector<int>& nums, int target) {\n        \n    }\n};\n"
        );
    }

    #[test]
    fn types_and_helpers() {
        let s = sig(json!({"kind": "function", "name": "f",
            "params": [{"name": "a", "type": "ListNode"}, {"name": "b", "type": "TreeNode"},
                       {"name": "c", "type": "Node"}, {"name": "d", "type": "char[][]"},
                       {"name": "e", "type": "string"}, {"name": "g", "type": "long"},
                       {"name": "h", "type": "bool"}],
            "returns": "double"}));
        let code = CppDriver.starter_code(&s);
        assert!(code.contains("struct ListNode {"));
        assert!(code.contains("struct TreeNode {"));
        assert!(code.contains("class Node {"));
        assert!(code.contains(
            "double f(ListNode* a, TreeNode* b, Node* c, vector<vector<char>>& d, string e, long long g, bool h)"
        ));
    }

    #[test]
    fn special_starters_and_cases() {
        let bad = sig(
            json!({"kind": "function", "name": "firstBadVersion", "special": "bad_version",
            "params": [{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}],
            "returns": "int"}),
        );
        assert!(CppDriver
            .starter_code(&bad)
            .contains("// bool isBadVersion(int version);"));
        assert!(CppDriver
            .starter_code(&bad)
            .contains("int firstBadVersion(int n)"));
        assert_eq!(api_header(&bad), "bool isBadVersion(int version);\n");
        assert!(case_source(&bad).contains("lg_bad = a1;"));
        assert!(case_source(&bad).contains("sol->firstBadVersion(a0)"));

        let cycle = sig(
            json!({"kind": "function", "name": "hasCycle", "special": "list_cycle",
            "params": [{"name": "head", "type": "ListNode"}, {"name": "pos", "type": "int", "hidden": true}],
            "returns": "bool"}),
        );
        assert!(case_source(&cycle).contains("lg::link_cycle(a0, a1);"));

        let lca = sig(
            json!({"kind": "function", "name": "lca", "output_mode": "node_val",
            "params": [{"name": "root", "type": "TreeNode"}, {"name": "p", "type": "TreeNode", "ref_of": "root"}],
            "returns": "TreeNode"}),
        );
        let src = case_source(&lca);
        assert!(src.contains("auto a1 = lg::find(a0, args.items[1]);"));
        assert!(src.contains("res->val"));

        let inplace = sig(
            json!({"kind": "function", "name": "sortColors", "output_param": "nums",
            "params": [{"name": "nums", "type": "int[]"}], "returns": "void"}),
        );
        let src = case_source(&inplace);
        assert!(src.contains("    sol->sortColors(a0);\n") && src.contains("lg::enc(out, a0);"));

        let deep = sig(
            json!({"kind": "function", "name": "cloneGraph", "special": "deep_copy",
            "params": [{"name": "node", "type": "Node"}], "returns": "Node"}),
        );
        assert!(case_source(&deep).contains("lg::check_deep_copy(a0, res);"));
        assert_eq!(api_header(&deep), "");
    }

    #[test]
    fn design_and_codec() {
        let d = sig(json!({"kind": "design", "class": "LRUCache",
            "constructor": [{"name": "capacity", "type": "int"}],
            "methods": [{"name": "get", "params": [{"name": "key", "type": "int"}], "returns": "int"},
                        {"name": "put", "params": [{"name": "key", "type": "int"}, {"name": "value", "type": "int"}], "returns": "void"}]}));
        let code = CppDriver.starter_code(&d);
        assert!(code.starts_with("class LRUCache {\npublic:\n    LRUCache(int capacity) {"));
        assert!(code.contains(" * int param_1 = obj->get(key);\n * obj->put(key,value);\n"));
        let src = case_source(&d);
        assert!(src.contains("obj.reset(new LRUCache(c0));"));
        assert!(src.contains("else if (op == \"put\")"));

        let c = sig(json!({"kind": "codec", "class": "Codec",
            "encode": {"name": "serialize", "params": [{"name": "root", "type": "TreeNode"}], "returns": "string"},
            "decode": {"name": "deserialize", "params": [{"name": "data", "type": "string"}], "returns": "TreeNode"}}));
        let code = CppDriver.starter_code(&c);
        assert!(code.contains("    string serialize(TreeNode* root) {"));
        assert!(code.contains("// TreeNode* ans = deser.deserialize(ser.serialize(root));"));
        assert!(case_source(&c).contains("auto data = ser->serialize(v);"));
    }

    #[test]
    fn program_layout() {
        let s = sig(json!({"kind": "function", "name": "f", "params": [], "returns": "int"}));
        let p = CppDriver.program(&s, "class Solution {};", &Toolchains::default());
        assert!(p.files[0]
            .1
            .contains("#line 1 \"solution.cpp\"\nclass Solution {};\n"));
        assert_eq!(p.artifacts, vec!["prog".to_string()]);
        assert_eq!(p.compile.as_ref().unwrap()[0], "clang++");
        assert_eq!(p.run, vec!["./prog".to_string()]);
    }
}
