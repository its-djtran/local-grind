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

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        let mut level: Vec<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        while !level.is_empty() {
            depth += 1;
            let mut next = Vec::new();
            for node in &level {
                let node = node.borrow();
                if let Some(l) = &node.left {
                    next.push(Rc::clone(l));
                }
                if let Some(r) = &node.right {
                    next.push(Rc::clone(r));
                }
            }
            level = next;
        }
        depth
    }
}
