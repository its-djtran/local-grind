//! Language-neutral problem model: difficulty, value types and call signatures.
//!
//! Signatures are declared once per problem in content and every language driver
//! derives its starter code and test harness from them.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn as_str(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}

impl FromStr for Difficulty {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            _ => Err(Error::Invalid(format!("unknown difficulty `{s}`"))),
        }
    }
}

/// A value type as written in problem content, e.g. `int`, `string[]`, `TreeNode`, `int[][]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    Int,
    Long,
    Double,
    Bool,
    String,
    Char,
    ListNode,
    TreeNode,
    /// Undirected graph node (Clone Graph); serialized as an adjacency list.
    GraphNode,
    Void,
    List(Box<ValueType>),
}

impl ValueType {
    /// Innermost element type of nested lists (`int[][]` -> `int`).
    pub fn base(&self) -> &ValueType {
        match self {
            ValueType::List(inner) => inner.base(),
            other => other,
        }
    }

    /// Nesting depth of list types (`int` -> 0, `int[][]` -> 2).
    pub fn depth(&self) -> usize {
        match self {
            ValueType::List(inner) => 1 + inner.depth(),
            _ => 0,
        }
    }

    pub fn uses(&self, ty: &ValueType) -> bool {
        self.base() == ty
    }

    /// Whether a JSON value is a valid serialized instance of this type.
    pub fn accepts(&self, v: &serde_json::Value) -> bool {
        use serde_json::Value;
        let int_list = |v: &Value, nullable: bool| {
            v.as_array()
                .is_some_and(|a| a.iter().all(|x| x.is_i64() || (nullable && x.is_null())))
        };
        match self {
            ValueType::Int => v.as_i64().is_some_and(|n| i32::try_from(n).is_ok()),
            ValueType::Long => v.is_i64(),
            ValueType::Double => v.is_number(),
            ValueType::Bool => v.is_boolean(),
            ValueType::String => v.is_string(),
            ValueType::Char => v.as_str().is_some_and(|s| s.chars().count() == 1),
            ValueType::ListNode => int_list(v, false),
            ValueType::TreeNode => int_list(v, true),
            ValueType::GraphNode => v
                .as_array()
                .is_some_and(|a| a.iter().all(|x| int_list(x, false))),
            ValueType::Void => v.is_null(),
            ValueType::List(inner) => v
                .as_array()
                .is_some_and(|a| a.iter().all(|x| inner.accepts(x))),
        }
    }
}

impl FromStr for ValueType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        if let Some(inner) = s.strip_suffix("[]") {
            let inner: ValueType = inner.parse()?;
            if inner == ValueType::Void {
                return Err(Error::Invalid("void[] is not a valid type".into()));
            }
            return Ok(ValueType::List(Box::new(inner)));
        }
        Ok(match s {
            "int" => ValueType::Int,
            "long" => ValueType::Long,
            "double" => ValueType::Double,
            "bool" => ValueType::Bool,
            "string" => ValueType::String,
            "char" => ValueType::Char,
            "ListNode" => ValueType::ListNode,
            "TreeNode" => ValueType::TreeNode,
            "Node" => ValueType::GraphNode,
            "void" => ValueType::Void,
            _ => return Err(Error::Invalid(format!("unknown type `{s}`"))),
        })
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Int => f.write_str("int"),
            ValueType::Long => f.write_str("long"),
            ValueType::Double => f.write_str("double"),
            ValueType::Bool => f.write_str("bool"),
            ValueType::String => f.write_str("string"),
            ValueType::Char => f.write_str("char"),
            ValueType::ListNode => f.write_str("ListNode"),
            ValueType::TreeNode => f.write_str("TreeNode"),
            ValueType::GraphNode => f.write_str("Node"),
            ValueType::Void => f.write_str("void"),
            ValueType::List(inner) => write!(f, "{inner}[]"),
        }
    }
}

impl Serialize for ValueType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ValueType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: ValueType,
    /// The serialized value is a node value to look up in the tree parameter named here
    /// (Lowest Common Ancestor passes `p` and `q` this way).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_of: Option<String>,
    /// Present in the test input but not passed to the solution; consumed by a [`Special`].
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
}

impl Param {
    pub fn new(name: &str, ty: ValueType) -> Param {
        Param {
            name: name.into(),
            ty,
            ref_of: None,
            hidden: false,
        }
    }

    fn accepts(&self, v: &serde_json::Value) -> bool {
        if self.ref_of.is_some() {
            v.is_i64() || v.is_null()
        } else {
            self.ty.accepts(v)
        }
    }
}

/// Problem-specific wiring performed by the harness around the solution call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Special {
    /// The hidden int parameter is the index the list's tail links back to (-1: no cycle).
    ListCycle,
    /// The hidden int parameter is the first bad version; `isBadVersion` is provided.
    BadVersion,
    /// The returned graph must not share any node with the input graph.
    DeepCopy,
}

/// How a returned value is serialized for judging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputMode {
    /// A returned node is judged by its value alone.
    NodeVal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    #[serde(default)]
    pub params: Vec<Param>,
    pub returns: ValueType,
}

/// How test inputs are wired into the solution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Signature {
    /// `class Solution { ret name(params) }`. When `returns` is void, the judged output is the
    /// (mutated) parameter named by `output_param`.
    Function {
        name: String,
        params: Vec<Param>,
        returns: ValueType,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output_param: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        special: Option<Special>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output_mode: Option<OutputMode>,
    },
    /// A user-implemented class driven by an operation sequence, LeetCode style:
    /// input `[["LRUCache","put","get"], [[2],[1,1],[1]]]`, output `[null,null,1]`.
    Design {
        class: String,
        #[serde(default)]
        constructor: Vec<Param>,
        methods: Vec<Method>,
    },
    /// Round trip through a user codec: `Class().decode(Class().encode(input))`, using separate
    /// instances so no state can leak between the two halves.
    Codec {
        class: String,
        encode: Method,
        decode: Method,
    },
}

/// Range checks for hidden parameters, so a harness never builds an impossible input.
fn check_special(
    special: Option<Special>,
    params: &[Param],
    items: &[serde_json::Value],
) -> std::result::Result<(), String> {
    let hidden = params.iter().position(|p| p.hidden);
    let value = |i: usize| items[i].as_i64().unwrap_or_default();
    match (special, hidden) {
        (Some(Special::ListCycle), Some(h)) => {
            let len = params
                .iter()
                .position(|p| p.ty == ValueType::ListNode)
                .and_then(|i| items[i].as_array())
                .map_or(0, Vec::len) as i64;
            let pos = value(h);
            if pos < -1 || pos >= len {
                return Err(format!(
                    "`{}` must be -1 or a valid index into the list (0..{len})",
                    params[h].name
                ));
            }
            Ok(())
        }
        (Some(Special::BadVersion), Some(h)) => {
            let n = params
                .iter()
                .position(|p| !p.hidden && p.ty == ValueType::Int)
                .map_or(0, value);
            let bad = value(h);
            if !(1..=n).contains(&bad) {
                return Err(format!("`{}` must be between 1 and {n}", params[h].name));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

impl Signature {
    /// Number of serialized arguments per test case.
    pub fn arity(&self) -> usize {
        match self {
            Signature::Function { params, .. } => params.len(),
            Signature::Design { .. } => 2,
            Signature::Codec { .. } => 1,
        }
    }

    /// Checks that a test's argument list matches this signature, describing the first problem.
    pub fn check_args(&self, args: &serde_json::Value) -> std::result::Result<(), String> {
        let Some(items) = args.as_array() else {
            return Err("arguments must be a JSON array".into());
        };
        if items.len() != self.arity() {
            return Err(format!(
                "expected {} argument(s), got {}",
                self.arity(),
                items.len()
            ));
        }
        match self {
            Signature::Function {
                params, special, ..
            } => {
                for (p, v) in params.iter().zip(items) {
                    if !p.accepts(v) {
                        return Err(format!("`{}` is not a valid {}", p.name, p.ty));
                    }
                }
                check_special(*special, params, items)
            }
            Signature::Design {
                class,
                constructor,
                methods,
            } => {
                let (Some(ops), Some(op_args)) = (items[0].as_array(), items[1].as_array()) else {
                    return Err("design input needs an operation list and an argument list".into());
                };
                if ops.len() != op_args.len() || ops.is_empty() {
                    return Err(
                        "operation and argument lists must be non-empty and equal length".into(),
                    );
                }
                for (i, (op, a)) in ops.iter().zip(op_args).enumerate() {
                    let name = op.as_str().unwrap_or_default();
                    let params = if i == 0 {
                        if name != class {
                            return Err(format!("first operation must be `{class}`"));
                        }
                        constructor
                    } else {
                        match methods.iter().find(|m| m.name == name) {
                            Some(m) => &m.params,
                            None => return Err(format!("unknown operation `{name}`")),
                        }
                    };
                    let a = a.as_array().map(Vec::as_slice).unwrap_or_default();
                    if a.len() != params.len() || !params.iter().zip(a).all(|(p, v)| p.accepts(v)) {
                        return Err(format!("bad arguments for operation {i} (`{name}`)"));
                    }
                }
                Ok(())
            }
            Signature::Codec { encode, .. } => {
                let p = &encode.params[0];
                if p.ty.accepts(&items[0]) {
                    Ok(())
                } else {
                    Err(format!("`{}` is not a valid {}", p.name, p.ty))
                }
            }
        }
    }

    /// Every value type mentioned by the signature; drivers use this to decide which helper
    /// classes (ListNode, TreeNode, ...) to emit.
    pub fn all_types(&self) -> Vec<&ValueType> {
        match self {
            Signature::Function {
                params, returns, ..
            } => params.iter().map(|p| &p.ty).chain([returns]).collect(),
            Signature::Design {
                constructor,
                methods,
                ..
            } => constructor
                .iter()
                .map(|p| &p.ty)
                .chain(
                    methods
                        .iter()
                        .flat_map(|m| m.params.iter().map(|p| &p.ty).chain([&m.returns])),
                )
                .collect(),
            Signature::Codec { encode, decode, .. } => [encode, decode]
                .into_iter()
                .flat_map(|m| m.params.iter().map(|p| &p.ty).chain([&m.returns]))
                .collect(),
        }
    }

    pub fn uses(&self, ty: &ValueType) -> bool {
        self.all_types().into_iter().any(|t| t.uses(ty))
    }

    /// Checks internal consistency (e.g. `output_param` names a real parameter).
    pub fn validate(&self) -> Result<()> {
        match self {
            Signature::Function {
                params,
                returns,
                output_param,
                special,
                output_mode,
                ..
            } => {
                let invalid = |m: String| Err(Error::Invalid(m));
                for p in params {
                    if let Some(target) = &p.ref_of {
                        let ok = p.ty == ValueType::TreeNode
                            && params.iter().any(|q| {
                                &q.name == target
                                    && q.ty == ValueType::TreeNode
                                    && q.ref_of.is_none()
                            });
                        if !ok {
                            return invalid(format!(
                                "`{}` must reference a TreeNode parameter",
                                p.name
                            ));
                        }
                    }
                }
                let hidden: Vec<&Param> = params.iter().filter(|p| p.hidden).collect();
                match special {
                    Some(Special::ListCycle | Special::BadVersion) => {
                        if hidden.len() != 1 || hidden[0].ty != ValueType::Int {
                            return invalid(
                                "this special needs exactly one hidden int parameter".into(),
                            );
                        }
                        if *special == Some(Special::ListCycle)
                            && !params.iter().any(|p| p.ty == ValueType::ListNode)
                        {
                            return invalid("list_cycle needs a ListNode parameter".into());
                        }
                    }
                    Some(Special::DeepCopy) | None => {
                        if !hidden.is_empty() {
                            return invalid(
                                "hidden parameters require list_cycle or bad_version".into(),
                            );
                        }
                        if *special == Some(Special::DeepCopy)
                            && !(returns.uses(&ValueType::GraphNode)
                                || returns.uses(&ValueType::ListNode)
                                || returns.uses(&ValueType::TreeNode))
                        {
                            return invalid("deep_copy needs a node return type".into());
                        }
                    }
                }
                if *output_mode == Some(OutputMode::NodeVal)
                    && !matches!(
                        returns,
                        ValueType::TreeNode | ValueType::ListNode | ValueType::GraphNode
                    )
                {
                    return invalid("node_val output needs a node return type".into());
                }
                if let Some(out) = output_param {
                    if *returns != ValueType::Void {
                        return Err(Error::Invalid(
                            "output_param requires a void return type".into(),
                        ));
                    }
                    if !params.iter().any(|p| &p.name == out) {
                        return Err(Error::Invalid(format!(
                            "output_param `{out}` is not a parameter"
                        )));
                    }
                } else if *returns == ValueType::Void {
                    return Err(Error::Invalid(
                        "void functions must declare output_param".into(),
                    ));
                }
                Ok(())
            }
            Signature::Design { methods, .. } => {
                if methods.is_empty() {
                    return Err(Error::Invalid("design problems need methods".into()));
                }
                Ok(())
            }
            Signature::Codec { encode, decode, .. } => {
                let ok = encode.params.len() == 1
                    && decode.params.len() == 1
                    && encode.returns == decode.params[0].ty
                    && decode.returns == encode.params[0].ty;
                if ok {
                    Ok(())
                } else {
                    Err(Error::Invalid(
                        "codec needs encode(x: T) -> U and decode(data: U) -> T".into(),
                    ))
                }
            }
        }
    }
}

/// How a program's output is compared with the expected output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum Comparator {
    #[default]
    Exact,
    /// Top-level list may be in any order.
    Unordered,
    /// List of lists where both the outer list and each inner list may be in any order
    /// (Group Anagrams, 3Sum, Combination Sum).
    UnorderedNested,
    /// Floating point comparison with absolute/relative tolerance.
    Float { tolerance: f64 },
    /// Problem-specific validator implemented in `judge::validators`.
    Validator { name: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_scalar_and_nested_types() {
        assert_eq!("int".parse::<ValueType>().unwrap(), ValueType::Int);
        let t: ValueType = "int[][]".parse().unwrap();
        assert_eq!(t.depth(), 2);
        assert_eq!(t.base(), &ValueType::Int);
        assert_eq!(t.to_string(), "int[][]");
        for s in [
            "long",
            "double",
            "bool",
            "string",
            "char",
            "ListNode",
            "TreeNode",
            "Node",
            "void",
            "ListNode[]",
        ] {
            assert_eq!(s.parse::<ValueType>().unwrap().to_string(), s);
        }
    }

    #[test]
    fn rejects_bad_types() {
        assert!("integer".parse::<ValueType>().is_err());
        assert!("void[]".parse::<ValueType>().is_err());
    }

    #[test]
    fn difficulty_roundtrip() {
        for d in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
            assert_eq!(d.as_str().parse::<Difficulty>().unwrap(), d);
        }
        assert_eq!("HARD".parse::<Difficulty>().unwrap(), Difficulty::Hard);
        assert!("extreme".parse::<Difficulty>().is_err());
    }

    #[test]
    fn value_type_serde() {
        let p: Param = serde_json::from_str(r#"{"name":"a","type":"string[]"}"#).unwrap();
        assert_eq!(p.ty, ValueType::List(Box::new(ValueType::String)));
        assert_eq!(
            serde_json::to_string(&p).unwrap(),
            r#"{"name":"a","type":"string[]"}"#
        );
        assert!(serde_json::from_str::<Param>(r#"{"name":"a","type":"x"}"#).is_err());
    }

    fn func(returns: &str, output_param: Option<&str>) -> Signature {
        Signature::Function {
            name: "f".into(),
            params: vec![Param::new("head", "ListNode".parse().unwrap())],
            returns: returns.parse().unwrap(),
            output_param: output_param.map(String::from),
            special: None,
            output_mode: None,
        }
    }

    #[test]
    fn signature_validation() {
        assert!(func("int", None).validate().is_ok());
        assert!(func("void", Some("head")).validate().is_ok());
        assert!(func("void", None).validate().is_err());
        assert!(func("void", Some("nope")).validate().is_err());
        assert!(func("int", Some("head")).validate().is_err());
        let design = Signature::Design {
            class: "X".into(),
            constructor: vec![],
            methods: vec![],
        };
        assert!(design.validate().is_err());
    }

    #[test]
    fn signature_type_queries() {
        let sig = func("TreeNode", None);
        assert!(sig.uses(&ValueType::ListNode));
        assert!(sig.uses(&ValueType::TreeNode));
        assert!(!sig.uses(&ValueType::GraphNode));
        let design = Signature::Design {
            class: "LRUCache".into(),
            constructor: vec![Param::new("capacity", ValueType::Int)],
            methods: vec![Method {
                name: "get".into(),
                params: vec![Param::new("key", ValueType::Int)],
                returns: "int[]".parse().unwrap(),
            }],
        };
        assert!(design.validate().is_ok());
        assert_eq!(design.all_types().len(), 3);
        let json = serde_json::to_string(&design).unwrap();
        assert_eq!(serde_json::from_str::<Signature>(&json).unwrap(), design);
    }

    #[test]
    fn accepts_values() {
        use serde_json::json;
        let t = |s: &str| s.parse::<ValueType>().unwrap();
        assert!(t("int").accepts(&json!(5)));
        assert!(!t("int").accepts(&json!(5_000_000_000i64)));
        assert!(t("long").accepts(&json!(5_000_000_000i64)));
        assert!(t("double").accepts(&json!(1.5)));
        assert!(t("bool").accepts(&json!(true)));
        assert!(t("string").accepts(&json!("x")));
        assert!(t("char").accepts(&json!("é")));
        assert!(!t("char").accepts(&json!("ab")));
        assert!(t("ListNode").accepts(&json!([1, 2])));
        assert!(!t("ListNode").accepts(&json!([1, null])));
        assert!(t("TreeNode").accepts(&json!([1, null, 2])));
        assert!(t("Node").accepts(&json!([[2], [1]])));
        assert!(t("void").accepts(&json!(null)));
        assert!(t("char[][]").accepts(&json!([["a"], ["b", "c"]])));
        assert!(!t("int[]").accepts(&json!([1, "2"])));
        assert!(!t("int[]").accepts(&json!(1)));
    }

    #[test]
    fn checks_args() {
        use serde_json::json;
        let f = func("int", None);
        assert_eq!(f.arity(), 1);
        assert!(f.check_args(&json!([[1, 2]])).is_ok());
        assert!(f
            .check_args(&json!([[1, 2], 3]))
            .unwrap_err()
            .contains("expected 1"));
        assert!(f.check_args(&json!(["x"])).unwrap_err().contains("`head`"));
        assert!(f.check_args(&json!({})).is_err());
        let d: Signature = serde_json::from_value(json!({"kind": "design", "class": "C",
            "constructor": [{"name": "n", "type": "int"}],
            "methods": [{"name": "add", "params": [{"name": "x", "type": "int"}], "returns": "void"}]}))
        .unwrap();
        assert!(d.check_args(&json!([["C", "add"], [[1], [2]]])).is_ok());
        assert!(d
            .check_args(&json!([["D", "add"], [[1], [2]]]))
            .unwrap_err()
            .contains("first"));
        assert!(d
            .check_args(&json!([["C", "pop"], [[1], []]]))
            .unwrap_err()
            .contains("unknown"));
        assert!(d
            .check_args(&json!([["C", "add"], [[1], ["x"]]]))
            .unwrap_err()
            .contains("bad arguments"));
        assert!(d.check_args(&json!([["C"], []])).is_err());
        assert!(d.check_args(&json!([1, 2])).is_err());
    }

    fn sig(v: serde_json::Value) -> Signature {
        serde_json::from_value(v).unwrap()
    }

    #[test]
    fn special_signatures_validate() {
        use serde_json::json;
        let lca = |p_ty: &str, target: &str, mode: &str| {
            sig(
                json!({"kind": "function", "name": "f", "returns": "TreeNode", "output_mode": mode,
                "params": [{"name": "root", "type": "TreeNode"}, {"name": "p", "type": p_ty, "ref_of": target}]}),
            )
        };
        assert!(lca("TreeNode", "root", "node_val").validate().is_ok());
        assert!(lca("int", "root", "node_val").validate().is_err());
        assert!(lca("TreeNode", "nope", "node_val").validate().is_err());
        let ok = lca("TreeNode", "root", "node_val");
        assert!(ok.check_args(&json!([[2, 1, 3], 1])).is_ok());
        assert!(ok.check_args(&json!([[2, 1, 3], "x"])).is_err());
        let bad_mode = sig(
            json!({"kind": "function", "name": "f", "returns": "int", "output_mode": "node_val", "params": []}),
        );
        assert!(bad_mode.validate().is_err());

        let special = |name: &str, params: serde_json::Value, returns: &str| {
            sig(
                json!({"kind": "function", "name": "f", "special": name, "params": params, "returns": returns}),
            )
        };
        let hidden_int = json!([{"name": "head", "type": "ListNode"}, {"name": "pos", "type": "int", "hidden": true}]);
        assert!(special("list_cycle", hidden_int.clone(), "bool")
            .validate()
            .is_ok());
        assert!(special(
            "bad_version",
            json!([{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}]),
            "int"
        )
        .validate()
        .is_ok());
        assert!(special(
            "list_cycle",
            json!([{"name": "n", "type": "int"}, {"name": "pos", "type": "int", "hidden": true}]),
            "bool"
        )
        .validate()
        .is_err());
        assert!(
            special("bad_version", json!([{"name": "n", "type": "int"}]), "int")
                .validate()
                .is_err()
        );
        assert!(special("deep_copy", hidden_int, "ListNode")
            .validate()
            .is_err());
        assert!(special(
            "deep_copy",
            json!([{"name": "node", "type": "Node"}]),
            "Node"
        )
        .validate()
        .is_ok());
        assert!(special(
            "deep_copy",
            json!([{"name": "node", "type": "Node"}]),
            "int"
        )
        .validate()
        .is_err());

        let codec = |ret: &str| {
            sig(json!({"kind": "codec", "class": "Codec",
                "encode": {"name": "serialize", "params": [{"name": "root", "type": "TreeNode"}], "returns": "string"},
                "decode": {"name": "deserialize", "params": [{"name": "data", "type": "string"}], "returns": ret}}))
        };
        let c = codec("TreeNode");
        assert!(c.validate().is_ok());
        assert!(codec("int").validate().is_err());
        assert_eq!(c.arity(), 1);
        assert_eq!(c.all_types().len(), 4);
        assert!(c.check_args(&json!([[1, null, 2]])).is_ok());
        assert!(c.check_args(&json!(["x"])).unwrap_err().contains("`root`"));
        assert!(Param::new("x", ValueType::Int).accepts(&json!(1)));
    }

    #[test]
    fn checks_special_parameters() {
        use serde_json::json;
        let cycle = sig(
            json!({"kind": "function", "name": "f", "special": "list_cycle", "returns": "bool",
            "params": [{"name": "head", "type": "ListNode"}, {"name": "pos", "type": "int", "hidden": true}]}),
        );
        assert!(cycle.check_args(&json!([[1, 2, 3], 2])).is_ok());
        assert!(cycle.check_args(&json!([[1, 2, 3], -1])).is_ok());
        assert!(cycle.check_args(&json!([[], -1])).is_ok());
        assert!(cycle
            .check_args(&json!([[1, 2, 3], 3]))
            .unwrap_err()
            .contains("`pos`"));
        assert!(cycle.check_args(&json!([[1], -2])).is_err());
        let bad = sig(
            json!({"kind": "function", "name": "f", "special": "bad_version", "returns": "int",
            "params": [{"name": "n", "type": "int"}, {"name": "bad", "type": "int", "hidden": true}]}),
        );
        assert!(bad.check_args(&json!([5, 5])).is_ok());
        assert!(bad
            .check_args(&json!([5, 6]))
            .unwrap_err()
            .contains("between 1 and 5"));
        assert!(bad.check_args(&json!([5, 0])).is_err());
    }

    #[test]
    fn comparator_serde() {
        let c: Comparator = serde_json::from_str(r#"{"mode":"float","tolerance":1e-5}"#).unwrap();
        assert_eq!(c, Comparator::Float { tolerance: 1e-5 });
        assert_eq!(Comparator::default(), Comparator::Exact);
    }
}
