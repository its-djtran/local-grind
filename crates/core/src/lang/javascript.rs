use super::{Driver, Language, Program, Toolchains};
use crate::types::{Method, Param, Signature, Special, ValueType};

const HARNESS: &str = include_str!("../../drivers/javascript/lg_harness.js");

pub struct JavaScriptDriver;

fn js_type(t: &ValueType) -> String {
    match t {
        ValueType::Int | ValueType::Long | ValueType::Double => "number".into(),
        ValueType::Bool => "boolean".into(),
        ValueType::String | ValueType::Char => "string".into(),
        ValueType::ListNode => "ListNode".into(),
        ValueType::TreeNode => "TreeNode".into(),
        ValueType::GraphNode => "_Node".into(),
        ValueType::Void => "void".into(),
        ValueType::List(inner) => format!("{}[]", js_type(inner)),
    }
}

fn visible(ps: &[Param]) -> Vec<&Param> {
    ps.iter().filter(|p| !p.hidden).collect()
}

fn jsdoc(params: &[&Param], returns: Option<&ValueType>) -> String {
    let mut out = String::from("/**\n");
    for p in params {
        out.push_str(&format!(" * @param {{{}}} {}\n", js_type(&p.ty), p.name));
    }
    if let Some(r) = returns {
        out.push_str(&format!(" * @return {{{}}}\n", js_type(r)));
    }
    out.push_str(" */\n");
    out
}

fn names(params: &[&Param]) -> String {
    params
        .iter()
        .map(|p| p.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn helper_comments(sig: &Signature) -> String {
    let mut out = String::new();
    if sig.uses(&ValueType::ListNode) {
        out.push_str(
            "/**\n * Definition for singly-linked list.\n * function ListNode(val, next) {\n\
             \x20*     this.val = (val===undefined ? 0 : val)\n\
             \x20*     this.next = (next===undefined ? null : next)\n * }\n */\n",
        );
    }
    if sig.uses(&ValueType::TreeNode) {
        out.push_str(
            "/**\n * Definition for a binary tree node.\n * function TreeNode(val, left, right) {\n\
             \x20*     this.val = (val===undefined ? 0 : val)\n\
             \x20*     this.left = (left===undefined ? null : left)\n\
             \x20*     this.right = (right===undefined ? null : right)\n * }\n */\n",
        );
    }
    if sig.uses(&ValueType::GraphNode) {
        out.push_str(
            "/**\n * // Definition for a _Node.\n * function _Node(val, neighbors) {\n\
             \x20*    this.val = val === undefined ? 0 : val;\n\
             \x20*    this.neighbors = neighbors === undefined ? [] : neighbors;\n * };\n */\n",
        );
    }
    out
}

fn method_stub(class: &str, m: &Method) -> String {
    let ps = visible(&m.params);
    format!(
        "{}{class}.prototype.{} = function({}) {{\n    \n}};\n",
        jsdoc(&ps, Some(&m.returns)),
        m.name,
        names(&ps)
    )
}

impl Driver for JavaScriptDriver {
    fn language(&self) -> Language {
        Language::JavaScript
    }

    fn starter_code(&self, sig: &Signature) -> String {
        let mut out = helper_comments(sig);
        if !out.is_empty() {
            out.push('\n');
        }
        match sig {
            Signature::Function {
                name,
                params,
                returns,
                special,
                ..
            } => {
                let ps = visible(params);
                if *special == Some(Special::BadVersion) {
                    out.push_str(
                        "/**\n * Definition for isBadVersion()\n *\n * @param {integer} version number\n\
                         \x20* @return {boolean} whether the version is bad\n\
                         \x20* isBadVersion = function(version) {\n *     ...\n * };\n */\n\n\
                         /**\n * @param {function} isBadVersion()\n * @return {function}\n */\n\
                         var solution = function(isBadVersion) {\n",
                    );
                    out.push_str(&format!(
                        "    {}    return function({}) {{\n        \n    }};\n}};\n",
                        jsdoc(&ps, Some(returns))
                            .replace('\n', "\n    ")
                            .trim_end_matches(' '),
                        names(&ps)
                    ));
                } else {
                    out.push_str(&format!(
                        "{}var {name} = function({}) {{\n    \n}};\n",
                        jsdoc(&ps, Some(returns)),
                        names(&ps)
                    ));
                }
            }
            Signature::Design {
                class,
                constructor,
                methods,
            } => {
                let ps = visible(constructor);
                out.push_str(&format!(
                    "{}var {class} = function({}) {{\n    \n}};\n",
                    jsdoc(&ps, None),
                    names(&ps)
                ));
                for m in methods {
                    out.push('\n');
                    out.push_str(&method_stub(class, m));
                }
                out.push_str(&format!(
                    "\n/**\n * Your {class} object will be instantiated and called as such:\n * var obj = new {class}({})\n",
                    names(&ps)
                ));
                for (i, m) in methods.iter().enumerate() {
                    let call = format!("obj.{}({})", m.name, names(&visible(&m.params)));
                    if m.returns == ValueType::Void {
                        out.push_str(&format!(" * {call}\n"));
                    } else {
                        out.push_str(&format!(" * var param_{} = {call}\n", i + 1));
                    }
                }
                out.push_str(" */\n");
            }
            Signature::Codec { encode, decode, .. } => {
                for m in [encode, decode] {
                    let ps = visible(&m.params);
                    out.push_str(&format!(
                        "{}var {} = function({}) {{\n    \n}};\n\n",
                        jsdoc(&ps, Some(&m.returns)),
                        m.name,
                        names(&ps)
                    ));
                }
                out.push_str(&format!(
                    "/**\n * Your functions will be called as such:\n * {}({}(input));\n */\n",
                    decode.name, encode.name
                ));
            }
        }
        out
    }

    fn program(&self, sig: &Signature, user_code: &str, tools: &Toolchains) -> Program {
        let exe = tools.path(Language::JavaScript);
        let mut run = vec![exe.to_string_lossy().into_owned()];
        if super::is_jsc(&exe) {
            // Let JavaScriptCore use the deep main-thread stack the runner provides, and pass
            // the input file after `--` (jsc's script-argument separator).
            run.extend([
                "--maxPerThreadStackUsage=60000000".into(),
                "lg_harness.js".into(),
                "--".into(),
            ]);
        } else {
            run.push("lg_harness.js".into());
        }
        Program {
            files: vec![
                ("solution.js".into(), user_code.to_string()),
                ("lg_harness.js".into(), HARNESS.to_string()),
                (
                    "signature.json".into(),
                    serde_json::to_string(sig).expect("signature serializes"),
                ),
            ],
            compile: None,
            artifacts: vec![],
            run,
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
            JavaScriptDriver.starter_code(&s),
            "/**\n * @param {number[]} nums\n * @param {number} target\n * @return {number[]}\n */\nvar twoSum = function(nums, target) {\n    \n};\n"
        );
    }

    #[test]
    fn helpers_and_types() {
        let s = sig(json!({"kind": "function", "name": "f",
            "params": [{"name": "a", "type": "ListNode"}, {"name": "b", "type": "TreeNode"},
                       {"name": "c", "type": "Node"}, {"name": "d", "type": "char[][]"},
                       {"name": "e", "type": "bool"}],
            "returns": "double"}));
        let code = JavaScriptDriver.starter_code(&s);
        assert!(code.contains("function ListNode(val, next)"));
        assert!(code.contains("function TreeNode(val, left, right)"));
        assert!(code.contains("function _Node(val, neighbors)"));
        assert!(code.contains("@param {string[][]} d"));
        assert!(code.contains("@param {boolean} e"));
        assert!(code.contains("@param {_Node} c"));
        assert!(code.contains("@return {number}"));
    }

    #[test]
    fn bad_version_starter() {
        let s = sig(
            json!({"kind": "function", "name": "firstBadVersion", "special": "bad_version",
            "params": [{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}],
            "returns": "int"}),
        );
        let code = JavaScriptDriver.starter_code(&s);
        assert!(code.contains("var solution = function(isBadVersion) {"));
        assert!(code.contains("    return function(n) {"));
        assert!(!code.contains("bad}"));
    }

    #[test]
    fn design_and_codec_starters() {
        let d = sig(
            json!({"kind": "design", "class": "MinStack", "constructor": [],
            "methods": [{"name": "push", "params": [{"name": "val", "type": "int"}], "returns": "void"},
                        {"name": "getMin", "returns": "int"}]}),
        );
        let code = JavaScriptDriver.starter_code(&d);
        assert!(code.contains("var MinStack = function() {"));
        assert!(code.contains("MinStack.prototype.push = function(val) {"));
        assert!(code.contains(" * obj.push(val)\n * var param_2 = obj.getMin()\n"));
        let c = sig(json!({"kind": "codec", "class": "Codec",
            "encode": {"name": "serialize", "params": [{"name": "root", "type": "TreeNode"}], "returns": "string"},
            "decode": {"name": "deserialize", "params": [{"name": "data", "type": "string"}], "returns": "TreeNode"}}));
        let code = JavaScriptDriver.starter_code(&c);
        assert!(code.contains("var serialize = function(root) {"));
        assert!(code.contains("var deserialize = function(data) {"));
        assert!(code.contains("deserialize(serialize(input))"));
    }

    #[test]
    fn program_layout() {
        let s = sig(json!({"kind": "function", "name": "f", "params": [], "returns": "int"}));
        let p = JavaScriptDriver.program(&s, "code", &Toolchains::default());
        assert_eq!(p.files[0], ("solution.js".into(), "code".into()));
        assert_eq!(p.run, vec!["node".to_string(), "lg_harness.js".into()]);
        let jsc = Toolchains::default().with(Language::JavaScript, super::super::JSC_PATH);
        let p = JavaScriptDriver.program(&s, "code", &jsc);
        assert_eq!(p.run[2..], ["lg_harness.js".to_string(), "--".into()]);
    }
}
