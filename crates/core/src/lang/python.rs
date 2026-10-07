use super::{Driver, Language, Program, Toolchains};
use crate::types::{Param, Signature, Special, ValueType};

const HARNESS: &str = include_str!("../../drivers/python/lg_harness.py");

pub struct PythonDriver;

fn py_type(t: &ValueType) -> String {
    match t {
        ValueType::Int | ValueType::Long => "int".into(),
        ValueType::Double => "float".into(),
        ValueType::Bool => "bool".into(),
        ValueType::String | ValueType::Char => "str".into(),
        ValueType::ListNode => "Optional[ListNode]".into(),
        ValueType::TreeNode => "Optional[TreeNode]".into(),
        ValueType::GraphNode => "Optional['Node']".into(),
        ValueType::Void => "None".into(),
        ValueType::List(inner) => format!("List[{}]", py_type(inner)),
    }
}

fn params(ps: &[Param]) -> String {
    ps.iter()
        .filter(|p| !p.hidden)
        .map(|p| format!(", {}: {}", p.name, py_type(&p.ty)))
        .collect()
}

fn helper_comments(sig: &Signature) -> String {
    let mut out = String::new();
    if sig.uses(&ValueType::ListNode) {
        out.push_str(
            "# Definition for singly-linked list.\n\
             # class ListNode:\n\
             #     def __init__(self, val=0, next=None):\n\
             #         self.val = val\n\
             #         self.next = next\n",
        );
    }
    if sig.uses(&ValueType::TreeNode) {
        out.push_str(
            "# Definition for a binary tree node.\n\
             # class TreeNode:\n\
             #     def __init__(self, val=0, left=None, right=None):\n\
             #         self.val = val\n\
             #         self.left = left\n\
             #         self.right = right\n",
        );
    }
    if sig.uses(&ValueType::GraphNode) {
        out.push_str(
            "\"\"\"\n\
             # Definition for a Node.\n\
             class Node:\n\
             \x20   def __init__(self, val = 0, neighbors = None):\n\
             \x20       self.val = val\n\
             \x20       self.neighbors = neighbors if neighbors is not None else []\n\
             \"\"\"\n",
        );
    }
    out
}

impl Driver for PythonDriver {
    fn language(&self) -> Language {
        Language::Python
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
                special,
                ..
            } => {
                if *special == Some(Special::BadVersion) {
                    out.push_str(
                        "# The isBadVersion API is already defined for you.\n\
                         # def isBadVersion(version: int) -> bool:\n\n",
                    );
                }
                out.push_str(&format!(
                    "class Solution:\n    def {name}(self{}) -> {}:\n        ",
                    params(ps),
                    py_type(returns)
                ));
            }
            Signature::Design {
                class,
                constructor,
                methods,
            } => {
                out.push_str(&format!(
                    "class {class}:\n\n    def __init__(self{}):\n        \n",
                    params(constructor)
                ));
                for m in methods {
                    out.push_str(&format!(
                        "\n    def {}(self{}) -> {}:\n        \n",
                        m.name,
                        params(&m.params),
                        py_type(&m.returns)
                    ));
                }
                let ctor_args: Vec<&str> = constructor.iter().map(|p| p.name.as_str()).collect();
                out.push_str(&format!(
                    "\n\n# Your {class} object will be instantiated and called as such:\n# obj = {class}({})\n",
                    ctor_args.join(", ")
                ));
                for (i, m) in methods.iter().enumerate() {
                    let args: Vec<&str> = m.params.iter().map(|p| p.name.as_str()).collect();
                    let call = format!("obj.{}({})", m.name, args.join(","));
                    if m.returns == ValueType::Void {
                        out.push_str(&format!("# {call}\n"));
                    } else {
                        out.push_str(&format!("# param_{} = {call}\n", i + 1));
                    }
                }
            }
            Signature::Codec {
                class,
                encode,
                decode,
            } => {
                out.push_str(&format!("class {class}:\n"));
                for m in [encode, decode] {
                    out.push_str(&format!(
                        "\n    def {}(self{}) -> {}:\n        \n",
                        m.name,
                        params(&m.params),
                        py_type(&m.returns)
                    ));
                }
                let input = &encode.params[0].name;
                out.push_str(&format!(
                    "\n\n# Your {class} object will be instantiated and called as such:\n\
                     # ser = {class}()\n# deser = {class}()\n# ans = deser.{}(ser.{}({input}))\n",
                    decode.name, encode.name
                ));
            }
        }
        out
    }

    fn program(&self, sig: &Signature, user_code: &str, tools: &Toolchains) -> Program {
        Program {
            files: vec![
                ("solution.py".into(), user_code.to_string()),
                ("lg_harness.py".into(), HARNESS.to_string()),
                (
                    "signature.json".into(),
                    serde_json::to_string(sig).expect("signature serializes"),
                ),
            ],
            compile: None,
            artifacts: vec![],
            run: vec![
                tools.path(Language::Python).to_string_lossy().into_owned(),
                "-X".into(),
                "utf8".into(),
                "lg_harness.py".into(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Method;

    fn p(name: &str, ty: &str) -> Param {
        Param::new(name, ty.parse().unwrap())
    }

    #[test]
    fn function_starter() {
        let sig = Signature::Function {
            name: "twoSum".into(),
            params: vec![p("nums", "int[]"), p("target", "int")],
            returns: "int[]".parse().unwrap(),
            output_param: None,
            special: None,
            output_mode: None,
        };
        assert_eq!(
            PythonDriver.starter_code(&sig),
            "class Solution:\n    def twoSum(self, nums: List[int], target: int) -> List[int]:\n        "
        );
    }

    #[test]
    fn helper_definitions_in_starter() {
        let sig = Signature::Function {
            name: "f".into(),
            params: vec![p("a", "ListNode"), p("b", "TreeNode"), p("c", "Node")],
            returns: "double".parse().unwrap(),
            output_param: None,
            special: None,
            output_mode: None,
        };
        let code = PythonDriver.starter_code(&sig);
        assert!(code.contains("# class ListNode:"));
        assert!(code.contains("# class TreeNode:"));
        assert!(code.contains("class Node:"));
        assert!(code.contains(
            "a: Optional[ListNode], b: Optional[TreeNode], c: Optional['Node']) -> float"
        ));
    }

    #[test]
    fn design_starter() {
        let sig = Signature::Design {
            class: "MinStack".into(),
            constructor: vec![],
            methods: vec![
                Method {
                    name: "push".into(),
                    params: vec![p("val", "int")],
                    returns: ValueType::Void,
                },
                Method {
                    name: "top".into(),
                    params: vec![],
                    returns: ValueType::Int,
                },
                Method {
                    name: "ok".into(),
                    params: vec![p("w", "string"), p("c", "char[]")],
                    returns: "bool".parse().unwrap(),
                },
            ],
        };
        let code = PythonDriver.starter_code(&sig);
        assert!(code.starts_with("class MinStack:\n\n    def __init__(self):\n"));
        assert!(code.contains("    def push(self, val: int) -> None:\n"));
        assert!(code.contains("# obj = MinStack()\n# obj.push(val)\n# param_2 = obj.top()\n"));
        assert!(code.contains("def ok(self, w: str, c: List[str]) -> bool:"));
    }

    #[test]
    fn special_starters() {
        let bad: Signature = serde_json::from_value(serde_json::json!({"kind": "function",
            "name": "firstBadVersion", "special": "bad_version", "returns": "int",
            "params": [{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}]}))
        .unwrap();
        let code = PythonDriver.starter_code(&bad);
        assert!(code.starts_with("# The isBadVersion API is already defined for you."));
        assert!(code.contains("def firstBadVersion(self, n: int) -> int:"));

        let codec: Signature = serde_json::from_value(serde_json::json!({"kind": "codec", "class": "Codec",
            "encode": {"name": "serialize", "params": [{"name": "root", "type": "TreeNode"}], "returns": "string"},
            "decode": {"name": "deserialize", "params": [{"name": "data", "type": "string"}], "returns": "TreeNode"}}))
        .unwrap();
        let code = PythonDriver.starter_code(&codec);
        assert!(code.contains("# class TreeNode:"));
        assert!(code
            .contains("class Codec:\n\n    def serialize(self, root: Optional[TreeNode]) -> str:"));
        assert!(code.contains("def deserialize(self, data: str) -> Optional[TreeNode]:"));
        assert!(code.contains("# ans = deser.deserialize(ser.serialize(root))"));
    }

    #[test]
    fn program_layout() {
        let sig = Signature::Function {
            name: "f".into(),
            params: vec![],
            returns: ValueType::Int,
            output_param: None,
            special: None,
            output_mode: None,
        };
        let prog = PythonDriver.program(&sig, "code", &Toolchains::default());
        assert_eq!(prog.files.len(), 3);
        assert_eq!(prog.files[0], ("solution.py".into(), "code".into()));
        assert!(prog.files[1].1.contains("SENTINEL = \"\\x1eLG \""));
        assert!(prog.compile.is_none());
        assert_eq!(prog.run[0], "python3");
    }
}
