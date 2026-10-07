// Local Grind Rust harness. The user's solution.rs and the generated lg_case.rs are included
// into `mod user` below; this file provides the LeetCode node types, JSON decoding/encoding and
// the test loop (see crates/core/src/lang/rust.rs).
#![allow(dead_code, unused_imports, unused_variables, unused_mut, non_snake_case, unused_macros)]

use std::cell::RefCell;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::*;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub struct Solution;

pub mod lg {
    use super::{ListNode, TreeNode};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicI64, Ordering};

    pub static BAD: AtomicI64 = AtomicI64::new(0);
    pub const MAX_NODES: usize = 1_000_000;

    #[derive(Debug, Clone)]
    pub enum Json {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<Json>),
    }

    impl Json {
        pub fn arr(&self) -> &[Json] {
            match self {
                Json::Arr(v) => v,
                _ => &[],
            }
        }
        pub fn text(&self) -> &str {
            match self {
                Json::Num(s) | Json::Str(s) => s,
                _ => "",
            }
        }
    }

    pub fn parse(s: &str) -> Json {
        let b: Vec<char> = s.chars().collect();
        let mut i = 0;
        value(&b, &mut i)
    }

    fn ws(b: &[char], i: &mut usize) {
        while *i < b.len() && b[*i].is_whitespace() {
            *i += 1;
        }
    }

    fn hex4(b: &[char], i: usize) -> u32 {
        u32::from_str_radix(&b[i..i + 4].iter().collect::<String>(), 16).unwrap_or(0xFFFD)
    }

    fn value(b: &[char], i: &mut usize) -> Json {
        ws(b, i);
        match b[*i] {
            '[' => {
                *i += 1;
                let mut v = Vec::new();
                ws(b, i);
                if b[*i] == ']' {
                    *i += 1;
                    return Json::Arr(v);
                }
                loop {
                    v.push(value(b, i));
                    ws(b, i);
                    let c = b[*i];
                    *i += 1;
                    if c == ']' {
                        return Json::Arr(v);
                    }
                }
            }
            '"' => {
                *i += 1;
                let mut s = String::new();
                while b[*i] != '"' {
                    if b[*i] == '\\' {
                        *i += 1;
                        match b[*i] {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            'b' => s.push('\u{8}'),
                            'f' => s.push('\u{c}'),
                            'u' => {
                                let mut cp = hex4(b, *i + 1);
                                *i += 4;
                                if (0xD800..0xDC00).contains(&cp) && b.get(*i + 1) == Some(&'\\') {
                                    let lo = hex4(b, *i + 3);
                                    cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                                    *i += 6;
                                }
                                s.push(char::from_u32(cp).unwrap_or('\u{FFFD}'));
                            }
                            c => s.push(c),
                        }
                        *i += 1;
                    } else {
                        s.push(b[*i]);
                        *i += 1;
                    }
                }
                *i += 1;
                Json::Str(s)
            }
            'n' => {
                *i += 4;
                Json::Null
            }
            't' => {
                *i += 4;
                Json::Bool(true)
            }
            'f' => {
                *i += 5;
                Json::Bool(false)
            }
            _ => {
                let start = *i;
                while *i < b.len() && (b[*i].is_ascii_digit() || "+-.eE".contains(b[*i])) {
                    *i += 1;
                }
                Json::Num(b[start..*i].iter().collect())
            }
        }
    }

    pub trait Dec: Sized {
        fn dec(j: &Json) -> Self;
    }
    impl Dec for i32 {
        fn dec(j: &Json) -> Self {
            j.text().parse::<i64>().unwrap() as i32
        }
    }
    impl Dec for i64 {
        fn dec(j: &Json) -> Self {
            j.text().parse().unwrap()
        }
    }
    impl Dec for f64 {
        fn dec(j: &Json) -> Self {
            j.text().parse().unwrap()
        }
    }
    impl Dec for bool {
        fn dec(j: &Json) -> Self {
            matches!(j, Json::Bool(true))
        }
    }
    impl Dec for String {
        fn dec(j: &Json) -> Self {
            j.text().to_string()
        }
    }
    impl Dec for char {
        fn dec(j: &Json) -> Self {
            j.text().chars().next().unwrap_or('\0')
        }
    }
    impl<T: Dec> Dec for Vec<T> {
        fn dec(j: &Json) -> Self {
            j.arr().iter().map(T::dec).collect()
        }
    }
    impl Dec for Option<Box<ListNode>> {
        fn dec(j: &Json) -> Self {
            let mut head = None;
            for x in j.arr().iter().rev() {
                let mut n = Box::new(ListNode::new(i32::dec(x)));
                n.next = head;
                head = Some(n);
            }
            head
        }
    }
    impl Dec for Option<Rc<RefCell<TreeNode>>> {
        fn dec(j: &Json) -> Self {
            let v = j.arr();
            if v.is_empty() || matches!(v[0], Json::Null) {
                return None;
            }
            let root = Rc::new(RefCell::new(TreeNode::new(i32::dec(&v[0]))));
            let mut q = VecDeque::from([root.clone()]);
            let mut i = 1;
            while let Some(n) = q.pop_front() {
                if i >= v.len() {
                    break;
                }
                if !matches!(v[i], Json::Null) {
                    let c = Rc::new(RefCell::new(TreeNode::new(i32::dec(&v[i]))));
                    n.borrow_mut().left = Some(c.clone());
                    q.push_back(c);
                }
                i += 1;
                if i < v.len() && !matches!(v[i], Json::Null) {
                    let c = Rc::new(RefCell::new(TreeNode::new(i32::dec(&v[i]))));
                    n.borrow_mut().right = Some(c.clone());
                    q.push_back(c);
                }
                i += 1;
            }
            Some(root)
        }
    }

    pub trait Enc {
        fn enc(&self, o: &mut String);
    }
    impl Enc for i32 {
        fn enc(&self, o: &mut String) {
            o.push_str(&self.to_string());
        }
    }
    impl Enc for i64 {
        fn enc(&self, o: &mut String) {
            o.push_str(&self.to_string());
        }
    }
    impl Enc for usize {
        fn enc(&self, o: &mut String) {
            o.push_str(&self.to_string());
        }
    }
    impl Enc for f64 {
        fn enc(&self, o: &mut String) {
            if self.is_finite() {
                o.push_str(&format!("{self:?}"));
            } else {
                o.push_str("null");
            }
        }
    }
    impl Enc for bool {
        fn enc(&self, o: &mut String) {
            o.push_str(if *self { "true" } else { "false" });
        }
    }
    pub fn enc_str(s: &str, o: &mut String) {
        o.push('"');
        for c in s.chars() {
            match c {
                '"' => o.push_str("\\\""),
                '\\' => o.push_str("\\\\"),
                '\n' => o.push_str("\\n"),
                '\t' => o.push_str("\\t"),
                '\r' => o.push_str("\\r"),
                c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
                c => o.push(c),
            }
        }
        o.push('"');
    }
    impl Enc for String {
        fn enc(&self, o: &mut String) {
            enc_str(self, o);
        }
    }
    impl Enc for &str {
        fn enc(&self, o: &mut String) {
            enc_str(self, o);
        }
    }
    impl Enc for char {
        fn enc(&self, o: &mut String) {
            enc_str(&self.to_string(), o);
        }
    }
    impl Enc for () {
        fn enc(&self, o: &mut String) {
            o.push_str("null");
        }
    }
    impl<T: Enc> Enc for Vec<T> {
        fn enc(&self, o: &mut String) {
            o.push('[');
            for (k, x) in self.iter().enumerate() {
                if k > 0 {
                    o.push(',');
                }
                x.enc(o);
            }
            o.push(']');
        }
    }
    impl Enc for Option<Box<ListNode>> {
        fn enc(&self, o: &mut String) {
            o.push('[');
            let mut cur = self.as_deref();
            let mut n = 0;
            while let Some(node) = cur {
                if n > 0 {
                    o.push(',');
                }
                n += 1;
                if n > MAX_NODES {
                    panic!("linked list output is too long");
                }
                o.push_str(&node.val.to_string());
                cur = node.next.as_deref();
            }
            o.push(']');
        }
    }
    impl Enc for Option<Rc<RefCell<TreeNode>>> {
        fn enc(&self, o: &mut String) {
            let mut order: Vec<Option<i32>> = Vec::new();
            let mut q = VecDeque::from([self.clone()]);
            while let Some(n) = q.pop_front() {
                match n {
                    None => order.push(None),
                    Some(n) => {
                        let n = n.borrow();
                        order.push(Some(n.val));
                        if order.len() > 2 * MAX_NODES {
                            panic!("tree output is too large (is there a cycle?)");
                        }
                        q.push_back(n.left.clone());
                        q.push_back(n.right.clone());
                    }
                }
            }
            while order.last() == Some(&None) {
                order.pop();
            }
            o.push('[');
            for (k, v) in order.iter().enumerate() {
                if k > 0 {
                    o.push(',');
                }
                match v {
                    Some(v) => o.push_str(&v.to_string()),
                    None => o.push_str("null"),
                }
            }
            o.push(']');
        }
    }

    /// Finds the node holding `value` inside `root` (shared, like LeetCode's `p` and `q`).
    pub fn find(root: &Option<Rc<RefCell<TreeNode>>>, value: &Json) -> Option<Rc<RefCell<TreeNode>>> {
        if matches!(value, Json::Null) {
            return None;
        }
        let target = i32::dec(value);
        let mut q = VecDeque::from([root.clone()]);
        while let Some(n) = q.pop_front() {
            if let Some(n) = n {
                if n.borrow().val == target {
                    return Some(n);
                }
                q.push_back(n.borrow().left.clone());
                q.push_back(n.borrow().right.clone());
            }
        }
        None
    }

    pub fn emit(line: &str) {
        use std::io::Write;
        let mut out = std::io::stdout().lock();
        let _ = out.write_all(line.as_bytes());
        let _ = out.flush();
    }
}

mod user {
    use super::*;
    include!("solution.rs");
    include!("lg_case.rs");
}

thread_local! {
    static PANIC: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn run(path: String) {
    std::panic::set_hook(Box::new(|info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Box<dyn Any>".to_string()
        };
        let location = info
            .location()
            .map(|l| format!(" at {}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_default();
        PANIC.with(|p| *p.borrow_mut() = Some(format!("thread panicked{location}:\n{msg}")));
    }));
    let input = std::fs::read_to_string(path).expect("input file");
    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let args = lg::parse(line);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| user::lg_case(&args))) {
            Ok((out, ns)) => lg::emit(&format!("\x1eLG {{\"i\":{i},\"ns\":{ns},\"out\":{out}}}\n")),
            Err(_) => {
                let msg = PANIC.with(|p| p.borrow_mut().take()).unwrap_or_default();
                let mut err = String::new();
                lg::enc_str(&msg, &mut err);
                lg::emit(&format!("\x1eLG {{\"i\":{i},\"error\":{err}}}\n"));
                return;
            }
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("input path");
    // A large stack lets recursive solutions handle deep (degenerate) trees.
    let child = std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(move || run(path))
        .expect("spawn");
    let _ = child.join();
}
