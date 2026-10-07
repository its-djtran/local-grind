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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        while !queue.is_empty() {
            let mut level = Vec::with_capacity(queue.len());
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                level.push(node.val);
                if let Some(l) = &node.left {
                    queue.push_back(Rc::clone(l));
                }
                if let Some(r) = &node.right {
                    queue.push_back(Rc::clone(r));
                }
            }
            levels.push(level);
        }
        levels
    }
}
