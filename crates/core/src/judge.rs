//! Output comparison. Drivers emit JSON; all judging happens here so every language
//! shares one implementation of each comparison mode.

use serde_json::Value;

use crate::types::Comparator;

/// Returns true when `actual` is an acceptable answer for a test with input `args`.
pub fn matches(cmp: &Comparator, args: &Value, expected: &Value, actual: &Value) -> bool {
    match cmp {
        Comparator::Exact => numbers_equal(expected, actual),
        Comparator::Unordered => unordered_eq(expected, actual, false),
        Comparator::UnorderedNested => unordered_eq(expected, actual, true),
        Comparator::Float { tolerance } => float_eq(expected, actual, *tolerance),
        Comparator::Validator { name } => validators::run(name, args, expected, actual),
    }
}

/// Structural equality that treats `2` and `2.0` as equal but is otherwise exact.
fn numbers_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if let (Some(x), Some(y)) = (x.as_i64(), y.as_i64()) {
                x == y
            } else {
                x.as_f64() == y.as_f64()
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| numbers_equal(a, b))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|w| numbers_equal(v, w)))
        }
        _ => a == b,
    }
}

fn canonical(v: &Value, nested: bool) -> String {
    match v {
        Value::Array(items) if nested => {
            let mut parts: Vec<String> = items.iter().map(|i| canonical(i, false)).collect();
            parts.sort();
            format!("[{}]", parts.join(","))
        }
        other => other.to_string(),
    }
}

fn unordered_eq(expected: &Value, actual: &Value, nested: bool) -> bool {
    let (Value::Array(e), Value::Array(a)) = (expected, actual) else {
        return numbers_equal(expected, actual);
    };
    if e.len() != a.len() {
        return false;
    }
    let mut ec: Vec<String> = e.iter().map(|v| canonical(v, nested)).collect();
    let mut ac: Vec<String> = a.iter().map(|v| canonical(v, nested)).collect();
    ec.sort();
    ac.sort();
    ec == ac
}

fn float_eq(expected: &Value, actual: &Value, tol: f64) -> bool {
    match (expected, actual) {
        (Value::Number(e), Value::Number(a)) => {
            let (e, a) = (
                e.as_f64().unwrap_or(f64::NAN),
                a.as_f64().unwrap_or(f64::NAN),
            );
            (e - a).abs() <= tol * e.abs().max(1.0)
        }
        (Value::Array(e), Value::Array(a)) => {
            e.len() == a.len() && e.iter().zip(a).all(|(x, y)| float_eq(x, y, tol))
        }
        _ => expected == actual,
    }
}

/// Validators for problems that accept more than one correct answer.
pub mod validators {
    use serde_json::Value;
    use std::collections::{HashMap, HashSet};

    pub const NAMES: &[&str] = &["alien_order", "longest_palindrome"];

    pub fn exists(name: &str) -> bool {
        NAMES.contains(&name)
    }

    pub fn run(name: &str, args: &Value, expected: &Value, actual: &Value) -> bool {
        match name {
            "alien_order" => alien_order(args, expected, actual),
            "longest_palindrome" => longest_palindrome(args, expected, actual),
            _ => false,
        }
    }

    /// Longest Palindromic Substring: any palindromic substring of `s` with the maximal length
    /// (the expected answer's length) is accepted.
    fn longest_palindrome(args: &Value, expected: &Value, actual: &Value) -> bool {
        let (Some(s), Some(expected), Some(actual)) = (
            args.get(0).and_then(Value::as_str),
            expected.as_str(),
            actual.as_str(),
        ) else {
            return false;
        };
        let chars: Vec<char> = actual.chars().collect();
        chars.len() == expected.chars().count()
            && s.contains(actual)
            && chars.iter().eq(chars.iter().rev())
    }

    /// Alien Dictionary: any ordering consistent with the word list is accepted; the expected
    /// output is "" exactly when no valid ordering exists.
    fn alien_order(args: &Value, expected: &Value, actual: &Value) -> bool {
        let (Some(expected), Some(actual)) = (expected.as_str(), actual.as_str()) else {
            return false;
        };
        if expected.is_empty() || actual.is_empty() {
            return expected == actual;
        }
        let Some(words) = args.get(0).and_then(Value::as_array) else {
            return false;
        };
        let words: Vec<&str> = words.iter().filter_map(Value::as_str).collect();
        let letters: HashSet<char> = words.iter().flat_map(|w| w.chars()).collect();
        let pos: HashMap<char, usize> = actual.chars().enumerate().map(|(i, c)| (c, i)).collect();
        if pos.len() != actual.chars().count() || letters != pos.keys().copied().collect() {
            return false;
        }
        words.windows(2).all(|pair| {
            let (a, b) = (pair[0], pair[1]);
            match a.chars().zip(b.chars()).find(|(x, y)| x != y) {
                Some((x, y)) => pos[&x] < pos[&y],
                None => a.len() <= b.len(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn check(cmp: Comparator, expected: Value, actual: Value) -> bool {
        matches(&cmp, &json!([]), &expected, &actual)
    }

    #[test]
    fn exact_comparison() {
        assert!(check(Comparator::Exact, json!([0, 1]), json!([0, 1])));
        assert!(!check(Comparator::Exact, json!([0, 1]), json!([1, 0])));
        assert!(check(Comparator::Exact, json!(2), json!(2.0)));
        assert!(check(Comparator::Exact, json!(2.5), json!(2.5)));
        assert!(!check(Comparator::Exact, json!(2), json!("2")));
        assert!(!check(Comparator::Exact, json!([1]), json!([1, 2])));
        assert!(check(Comparator::Exact, json!({"a": 1}), json!({"a": 1.0})));
        assert!(!check(Comparator::Exact, json!({"a": 1}), json!({"b": 1})));
        assert!(!check(
            Comparator::Exact,
            json!({"a": 1}),
            json!({"a": 1, "b": 2})
        ));
        assert!(check(Comparator::Exact, json!(null), json!(null)));
    }

    #[test]
    fn unordered_comparison() {
        assert!(check(Comparator::Unordered, json!([0, 1]), json!([1, 0])));
        assert!(!check(Comparator::Unordered, json!([0, 1]), json!([1, 1])));
        assert!(!check(Comparator::Unordered, json!([0, 1]), json!([1])));
        // Inner lists keep their order in plain unordered mode.
        assert!(!check(
            Comparator::Unordered,
            json!([[1, 2]]),
            json!([[2, 1]])
        ));
        assert!(check(Comparator::Unordered, json!(3), json!(3)));
    }

    #[test]
    fn unordered_nested_comparison() {
        let e = json!([["eat", "tea"], ["bat"]]);
        assert!(check(
            Comparator::UnorderedNested,
            e.clone(),
            json!([["bat"], ["tea", "eat"]])
        ));
        assert!(!check(
            Comparator::UnorderedNested,
            e,
            json!([["bat"], ["tea"]])
        ));
    }

    #[test]
    fn float_comparison() {
        let c = || Comparator::Float { tolerance: 1e-5 };
        assert!(check(c(), json!(2.5), json!(2.500001)));
        assert!(!check(c(), json!(2.5), json!(2.51)));
        assert!(check(c(), json!([1.0, 2.0]), json!([1.0, 2.000001])));
        assert!(!check(c(), json!([1.0]), json!([1.0, 2.0])));
        assert!(check(c(), json!(null), json!(null)));
        assert!(check(c(), json!(1e9), json!(1e9 + 1.0)));
    }

    #[test]
    fn alien_order_validator() {
        let cmp = Comparator::Validator {
            name: "alien_order".into(),
        };
        let args = json!([["wrt", "wrf", "er", "ett", "rftt"]]);
        let ok = |a: &str| matches(&cmp, &args, &json!("wertf"), &json!(a));
        assert!(ok("wertf"));
        assert!(!ok("wertff"));
        assert!(!ok("wetrf"));
        assert!(!ok("wert"));
        assert!(!ok(""));
        let prefix = json!([["abc", "ab"]]);
        assert!(matches(&cmp, &prefix, &json!(""), &json!("")));
        assert!(!matches(&cmp, &prefix, &json!(""), &json!("abc")));
        assert!(matches(
            &cmp,
            &json!([["ab", "abc"]]),
            &json!("abc"),
            &json!("cba")
        ));
        assert!(!matches(
            &cmp,
            &json!([["ab", "abc"]]),
            &json!("abc"),
            &json!(5)
        ));
        assert!(!matches(&cmp, &json!({}), &json!("abc"), &json!("abc")));
        assert!(validators::exists("alien_order"));
        let lp = Comparator::Validator {
            name: "longest_palindrome".into(),
        };
        let babad = json!(["babad"]);
        let ok = |a: Value| matches(&lp, &babad, &json!("bab"), &a);
        assert!(ok(json!("bab")));
        assert!(ok(json!("aba")));
        assert!(!ok(json!("ba")));
        assert!(!ok(json!("dad")));
        assert!(!ok(json!("abd")));
        assert!(!ok(json!(5)));
        assert!(!matches(&lp, &json!([]), &json!("a"), &json!("a")));
        assert!(!validators::exists("nope"));
        assert!(!validators::run("nope", &args, &json!(1), &json!(1)));
    }
}
