//! Rust driver. `lg_main.rs` (static) includes the user's `solution.rs` and a generated
//! `lg_case.rs` into one module, so private user items stay reachable and rustc reports errors
//! against the user's own line numbers.

use super::{Driver, Language, Program, Toolchains};
use crate::types::{Method, OutputMode, Param, Signature, Special, ValueType};

const PRELUDE: &str = include_str!("../../drivers/rust/lg_prelude.rs");

pub struct RustDriver;

/// `twoSum` -> `two_sum`, as LeetCode names Rust methods and parameters.
pub fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn rust_type(t: &ValueType) -> String {
    match t {
        ValueType::Int => "i32".into(),
        ValueType::Long => "i64".into(),
        ValueType::Double => "f64".into(),
        ValueType::Bool => "bool".into(),
        ValueType::String => "String".into(),
        ValueType::Char => "char".into(),
        ValueType::ListNode => "Option<Box<ListNode>>".into(),
        ValueType::TreeNode => "Option<Rc<RefCell<TreeNode>>>".into(),
        // Graph nodes are not offered in Rust (see `supports`).
        ValueType::GraphNode => "()".into(),
        ValueType::Void => "()".into(),
        ValueType::List(inner) => format!("Vec<{}>", rust_type(inner)),
    }
}

fn ret(t: &ValueType) -> String {
    if *t == ValueType::Void {
        String::new()
    } else {
        format!(" -> {}", rust_type(t))
    }
}

fn params(ps: &[Param], output_param: Option<&str>) -> String {
    ps.iter()
        .filter(|p| !p.hidden)
        .map(|p| {
            let ty = rust_type(&p.ty);
            if Some(p.name.as_str()) == output_param {
                format!("{}: &mut {ty}", snake(&p.name))
            } else {
                format!("{}: {ty}", snake(&p.name))
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn helper_comments(sig: &Signature) -> String {
    let mut out = String::new();
    if sig.uses(&ValueType::ListNode) {
        out.push_str(
            "// Definition for singly-linked list.\n// #[derive(PartialEq, Eq, Clone, Debug)]\n\
             // pub struct ListNode {\n//   pub val: i32,\n//   pub next: Option<Box<ListNode>>\n// }\n//\n\
             // impl ListNode {\n//   #[inline]\n//   fn new(val: i32) -> Self {\n//     ListNode {\n\
             //       next: None,\n//       val\n//     }\n//   }\n// }\n",
        );
    }
    if sig.uses(&ValueType::TreeNode) {
        out.push_str(
            "// Definition for a binary tree node.\n// #[derive(Debug, PartialEq, Eq)]\n\
             // pub struct TreeNode {\n//   pub val: i32,\n//   pub left: Option<Rc<RefCell<TreeNode>>>,\n\
             //   pub right: Option<Rc<RefCell<TreeNode>>>,\n// }\n//\n// impl TreeNode {\n//   #[inline]\n\
             //   pub fn new(val: i32) -> Self {\n//     TreeNode {\n//       val,\n//       left: None,\n\
             //       right: None\n//     }\n//   }\n// }\nuse std::rc::Rc;\nuse std::cell::RefCell;\n",
        );
    }
    out
}

fn method_stub(m: &Method) -> String {
    let ps = params(&m.params, None);
    let sep = if ps.is_empty() { "" } else { ", " };
    format!(
        "    fn {}(&self{sep}{ps}){} {{\n        \n    }}\n",
        snake(&m.name),
        ret(&m.returns)
    )
}

fn decode_params(ps: &[Param], src: &str, prefix: &str, indent: &str) -> String {
    let mut out = String::new();
    for (i, p) in ps.iter().enumerate() {
        if let Some(target) = &p.ref_of {
            let t = ps.iter().position(|q| &q.name == target).unwrap_or(0);
            out.push_str(&format!(
                "{indent}let {prefix}{i} = lg::find(&{prefix}{t}, &{src}[{i}]);\n"
            ));
        } else {
            out.push_str(&format!(
                "{indent}let mut {prefix}{i}: {} = lg::Dec::dec(&{src}[{i}]);\n",
                rust_type(&p.ty)
            ));
        }
    }
    out
}

fn call_args(ps: &[Param], prefix: &str, output_param: Option<&str>) -> String {
    ps.iter()
        .enumerate()
        .filter(|(_, p)| !p.hidden)
        .map(|(i, p)| {
            if Some(p.name.as_str()) == output_param {
                format!("&mut {prefix}{i}")
            } else {
                format!("{prefix}{i}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
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
            let bad_version = *special == Some(Special::BadVersion);
            if bad_version {
                out.push_str(
                    "impl Solution {\n    pub fn isBadVersion(&self, version: i32) -> bool {\n\
                     \x20       version as i64 >= lg::BAD.load(std::sync::atomic::Ordering::SeqCst)\n    }\n}\n\n",
                );
            }
            out.push_str("pub(crate) fn lg_case(args: &lg::Json) -> (String, u128) {\n    let a = args.arr();\n");
            out.push_str(&decode_params(ps, "a", "a", "    "));
            if let (true, Some(h)) = (bad_version, ps.iter().position(|p| p.hidden)) {
                out.push_str(&format!(
                    "    lg::BAD.store(a{h} as i64, std::sync::atomic::Ordering::SeqCst);\n"
                ));
            }
            let receiver = if bad_version {
                "Solution."
            } else {
                "Solution::"
            };
            let call = format!(
                "{receiver}{}({})",
                snake(name),
                call_args(ps, "a", output_param.as_deref())
            );
            out.push_str("    let t0 = std::time::Instant::now();\n");
            if *returns == ValueType::Void {
                out.push_str(&format!("    {call};\n"));
            } else {
                out.push_str(&format!("    let res = {call};\n"));
            }
            out.push_str(
                "    let ns = t0.elapsed().as_nanos();\n    let mut out = String::new();\n",
            );
            if let Some(param) = output_param {
                let idx = ps.iter().position(|p| &p.name == param).unwrap_or(0);
                out.push_str(&format!("    lg::Enc::enc(&a{idx}, &mut out);\n"));
            } else if *output_mode == Some(OutputMode::NodeVal) {
                out.push_str(
                    "    match &res {\n        Some(n) => lg::Enc::enc(&n.borrow().val, &mut out),\n\
                     \x20       None => out.push_str(\"null\"),\n    }\n",
                );
            } else {
                out.push_str("    lg::Enc::enc(&res, &mut out);\n");
            }
            out.push_str("    (out, ns)\n}\n");
        }
        Signature::Design {
            class,
            constructor,
            methods,
        } => {
            out.push_str(&format!(
                "pub(crate) fn lg_case(args: &lg::Json) -> (String, u128) {{\n\
                 \x20   let ops = args.arr()[0].arr();\n    let opargs = args.arr()[1].arr();\n\
                 \x20   let mut total: u128 = 0;\n    let mut out = String::from(\"[\");\n\
                 \x20   let mut obj: Option<{class}> = None;\n\
                 \x20   for (k, op) in ops.iter().enumerate() {{\n\
                 \x20       if k > 0 {{\n            out.push(',');\n        }}\n\
                 \x20       let a = opargs[k].arr();\n        if k == 0 {{\n"
            ));
            out.push_str(&decode_params(constructor, "a", "c", "            "));
            out.push_str(&format!(
                "            let t0 = std::time::Instant::now();\n            obj = Some({class}::new({}));\n\
                 \x20           total += t0.elapsed().as_nanos();\n            out.push_str(\"null\");\n\
                 \x20           continue;\n        }}\n        let o = obj.as_mut().unwrap();\n        match op.text() {{\n",
                call_args(constructor, "c", None)
            ));
            for m in methods {
                out.push_str(&format!("            \"{}\" => {{\n", m.name));
                out.push_str(&decode_params(&m.params, "a", "p", "                "));
                let call = format!("o.{}({})", snake(&m.name), call_args(&m.params, "p", None));
                out.push_str("                let t0 = std::time::Instant::now();\n");
                if m.returns == ValueType::Void {
                    out.push_str(&format!(
                        "                {call};\n                total += t0.elapsed().as_nanos();\n                out.push_str(\"null\");\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "                let r = {call};\n                total += t0.elapsed().as_nanos();\n                lg::Enc::enc(&r, &mut out);\n"
                    ));
                }
                out.push_str("            }\n");
            }
            out.push_str(
                "            other => panic!(\"unknown operation {other}\"),\n        }\n    }\n\
                 \x20   out.push(']');\n    (out, total)\n}\n",
            );
        }
        Signature::Codec {
            class,
            encode,
            decode,
        } => {
            out.push_str(&format!(
                "pub(crate) fn lg_case(args: &lg::Json) -> (String, u128) {{\n\
                 \x20   let v: {} = lg::Dec::dec(&args.arr()[0]);\n\
                 \x20   let ser = {class}::new();\n    let deser = {class}::new();\n\
                 \x20   let t0 = std::time::Instant::now();\n    let data = ser.{}(v);\n\
                 \x20   let res = deser.{}(data);\n    let ns = t0.elapsed().as_nanos();\n\
                 \x20   let mut out = String::new();\n    lg::Enc::enc(&res, &mut out);\n    (out, ns)\n}}\n",
                rust_type(&encode.params[0].ty),
                snake(&encode.name),
                snake(&decode.name)
            ));
        }
    }
    out
}

impl Driver for RustDriver {
    fn language(&self) -> Language {
        Language::Rust
    }

    /// LeetCode does not offer Rust for graph-node or cyclic-list problems: neither can be
    /// expressed with `Box`/`Rc` ownership without unsafe code.
    fn supports(&self, sig: &Signature) -> bool {
        let special = match sig {
            Signature::Function { special, .. } => *special,
            _ => None,
        };
        !sig.uses(&ValueType::GraphNode)
            && !matches!(special, Some(Special::ListCycle | Special::DeepCopy))
    }

    fn starter_code(&self, sig: &Signature) -> String {
        let mut out = helper_comments(sig);
        if !out.is_empty() {
            out.push('\n');
        }
        match sig {
            Signature::Function {
                name,
                params: ps,
                returns,
                output_param,
                special,
                ..
            } => {
                let ps = params(ps, output_param.as_deref());
                if *special == Some(Special::BadVersion) {
                    out.push_str(
                        "// The API isBadVersion is defined for you.\n// isBadVersion(version:i32)-> bool;\n\
                         // to call it use self.isBadVersion(version)\n\n",
                    );
                    out.push_str(&format!(
                        "impl Solution {{\n    pub fn {}(&self, {ps}){} {{\n        \n    }}\n}}\n",
                        snake(name),
                        ret(returns)
                    ));
                } else {
                    out.push_str(&format!(
                        "impl Solution {{\n    pub fn {}({ps}){} {{\n        \n    }}\n}}\n",
                        snake(name),
                        ret(returns)
                    ));
                }
            }
            Signature::Design {
                class,
                constructor,
                methods,
            } => {
                out.push_str(&format!(
                    "struct {class} {{\n\n}}\n\n\n/**\n * `&self` means the method takes an immutable reference.\n\
                     \x20* If you need a mutable reference, change it to `&mut self` instead.\n */\n\
                     impl {class} {{\n\n    fn new({}) -> Self {{\n        \n    }}\n",
                    params(constructor, None)
                ));
                for m in methods {
                    out.push_str("    \n");
                    out.push_str(&method_stub(m));
                }
                let ctor: Vec<String> = constructor.iter().map(|p| snake(&p.name)).collect();
                out.push_str(&format!(
                    "}}\n\n/*\n * Your {class} object will be instantiated and called as such:\n * let obj = {class}::new({});\n",
                    ctor.join(", ")
                ));
                for (i, m) in methods.iter().enumerate() {
                    let args: Vec<String> = m.params.iter().map(|p| snake(&p.name)).collect();
                    let call = format!("obj.{}({});", snake(&m.name), args.join(", "));
                    if m.returns == ValueType::Void {
                        out.push_str(&format!(" * {call}\n"));
                    } else {
                        out.push_str(&format!(
                            " * let ret_{}: {} = {call}\n",
                            i + 1,
                            rust_type(&m.returns)
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
                out.push_str(&format!(
                    "struct {class} {{\n\n}}\n\n/**\n * `&self` means the method takes an immutable reference.\n\
                     \x20* If you need a mutable reference, change it to `&mut self` instead.\n */\n\
                     impl {class} {{\n    fn new() -> Self {{\n        \n    }}\n\n"
                ));
                out.push_str(&method_stub(encode));
                out.push('\n');
                out.push_str(&method_stub(decode));
                out.push_str(&format!(
                    "}}\n\n/*\n * Your {class} object will be instantiated and called as such:\n\
                     \x20* let obj = {class}::new();\n * let data: String = obj.{}(strs);\n * let ans = obj.{}(data);\n */\n",
                    snake(&encode.name),
                    snake(&decode.name)
                ));
            }
        }
        out
    }

    fn program(&self, sig: &Signature, user_code: &str, tools: &Toolchains) -> Program {
        Program {
            files: vec![
                ("solution.rs".into(), user_code.to_string()),
                ("lg_main.rs".into(), PRELUDE.into()),
                ("lg_case.rs".into(), case_source(sig)),
            ],
            compile: Some(vec![
                tools.path(Language::Rust).to_string_lossy().into_owned(),
                "--edition".into(),
                "2021".into(),
                "-O".into(),
                "-C".into(),
                "overflow-checks=on".into(),
                "-A".into(),
                "warnings".into(),
                "-o".into(),
                "prog".into(),
                "lg_main.rs".into(),
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
    fn snake_case() {
        assert_eq!(snake("twoSum"), "two_sum");
        assert_eq!(snake("getMin"), "get_min");
        assert_eq!(snake("wordDict"), "word_dict");
        assert_eq!(snake("Codec"), "codec");
        assert_eq!(snake("n"), "n");
    }

    #[test]
    fn function_starters() {
        let s = sig(json!({"kind": "function", "name": "twoSum",
            "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
            "returns": "int[]"}));
        assert_eq!(
            RustDriver.starter_code(&s),
            "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        \n    }\n}\n"
        );
        let inplace = sig(
            json!({"kind": "function", "name": "sortColors", "output_param": "nums",
            "params": [{"name": "nums", "type": "int[]"}], "returns": "void"}),
        );
        assert!(RustDriver
            .starter_code(&inplace)
            .contains("pub fn sort_colors(nums: &mut Vec<i32>) {"));
        let tree = sig(json!({"kind": "function", "name": "maxDepth",
            "params": [{"name": "root", "type": "TreeNode"}, {"name": "head", "type": "ListNode"},
                       {"name": "x", "type": "long"}, {"name": "y", "type": "double"}, {"name": "z", "type": "char[][]"},
                       {"name": "w", "type": "bool"}, {"name": "s", "type": "string"}],
            "returns": "int"}));
        let code = RustDriver.starter_code(&tree);
        assert!(code.contains("use std::rc::Rc;\nuse std::cell::RefCell;"));
        assert!(code.contains("pub struct ListNode {"));
        assert!(code.contains(
            "root: Option<Rc<RefCell<TreeNode>>>, head: Option<Box<ListNode>>, x: i64, y: f64, z: Vec<Vec<char>>, w: bool, s: String"
        ));
        let bad = sig(
            json!({"kind": "function", "name": "firstBadVersion", "special": "bad_version",
            "params": [{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}],
            "returns": "int"}),
        );
        assert!(RustDriver
            .starter_code(&bad)
            .contains("pub fn first_bad_version(&self, n: i32) -> i32"));
        let src = case_source(&bad);
        assert!(src.contains("pub fn isBadVersion(&self, version: i32) -> bool"));
        assert!(src.contains("lg::BAD.store(a1 as i64"));
        assert!(src.contains("Solution.first_bad_version(a0)"));
    }

    #[test]
    fn design_and_codec_starters() {
        let d = sig(
            json!({"kind": "design", "class": "MinStack", "constructor": [],
            "methods": [{"name": "push", "params": [{"name": "val", "type": "int"}], "returns": "void"},
                        {"name": "getMin", "returns": "int"}]}),
        );
        let code = RustDriver.starter_code(&d);
        assert!(code.starts_with("struct MinStack {"));
        assert!(code.contains("    fn new() -> Self {"));
        assert!(code.contains("    fn push(&self, val: i32) {"));
        assert!(code.contains("    fn get_min(&self) -> i32 {"));
        assert!(code.contains(" * obj.push(val);\n * let ret_2: i32 = obj.get_min();\n"));
        // A trailing doc comment documents nothing and would not compile.
        assert!(code.contains("}\n\n/*\n * Your MinStack object"));
        let src = case_source(&d);
        assert!(src.contains("\"getMin\" => {") && src.contains("o.get_min()"));

        let c = sig(json!({"kind": "codec", "class": "Codec",
            "encode": {"name": "serialize", "params": [{"name": "root", "type": "TreeNode"}], "returns": "string"},
            "decode": {"name": "deserialize", "params": [{"name": "data", "type": "string"}], "returns": "TreeNode"}}));
        let code = RustDriver.starter_code(&c);
        assert!(code.contains("fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String"));
        assert!(case_source(&c).contains("let data = ser.serialize(v);"));
    }

    #[test]
    fn support_matrix() {
        let graph = sig(
            json!({"kind": "function", "name": "cloneGraph", "special": "deep_copy",
            "params": [{"name": "node", "type": "Node"}], "returns": "Node"}),
        );
        assert!(!RustDriver.supports(&graph));
        let cycle = sig(
            json!({"kind": "function", "name": "hasCycle", "special": "list_cycle",
            "params": [{"name": "head", "type": "ListNode"}, {"name": "pos", "type": "int", "hidden": true}],
            "returns": "bool"}),
        );
        assert!(!RustDriver.supports(&cycle));
        let lca = sig(
            json!({"kind": "function", "name": "lca", "output_mode": "node_val",
            "params": [{"name": "root", "type": "TreeNode"}, {"name": "p", "type": "TreeNode", "ref_of": "root"}],
            "returns": "TreeNode"}),
        );
        assert!(RustDriver.supports(&lca));
        assert!(case_source(&lca).contains("let a1 = lg::find(&a0, &a[1]);"));
        assert_eq!(rust_type(&ValueType::GraphNode), "()");
    }

    #[test]
    fn program_layout() {
        let s = sig(json!({"kind": "function", "name": "f", "params": [], "returns": "int"}));
        let p = RustDriver.program(&s, "impl Solution {}", &Toolchains::default());
        assert_eq!(
            p.files[0],
            ("solution.rs".into(), "impl Solution {}".into())
        );
        assert_eq!(p.compile.as_ref().unwrap()[0], "rustc");
        assert_eq!(p.artifacts, vec!["prog".to_string()]);
    }
}
