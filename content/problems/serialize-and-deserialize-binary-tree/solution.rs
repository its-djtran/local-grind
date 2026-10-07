// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out: Vec<String> = Vec::new();
        let mut queue = VecDeque::from([root]);
        while let Some(node) = queue.pop_front() {
            match node {
                None => out.push("#".to_string()),
                Some(n) => {
                    let n = n.borrow();
                    out.push(n.val.to_string());
                    queue.push_back(n.left.clone());
                    queue.push_back(n.right.clone());
                }
            }
        }
        out.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let tokens: Vec<&str> = data.split(',').collect();
        if data.is_empty() || tokens[0] == "#" {
            return None;
        }
        let make = |tok: &str| Rc::new(RefCell::new(TreeNode::new(tok.parse().unwrap())));
        let root = make(tokens[0]);
        let mut queue = VecDeque::from([Rc::clone(&root)]);
        let mut i = 1;
        while let Some(node) = queue.pop_front() {
            let mut n = node.borrow_mut();
            if tokens[i] != "#" {
                let child = make(tokens[i]);
                queue.push_back(Rc::clone(&child));
                n.left = Some(child);
            }
            if tokens[i + 1] != "#" {
                let child = make(tokens[i + 1]);
                queue.push_back(Rc::clone(&child));
                n.right = Some(child);
            }
            i += 2;
        }
        Some(root)
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans = obj.deserialize(data);
 */
