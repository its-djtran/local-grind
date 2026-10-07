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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn gain(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let left = gain(&n.left, best).max(0);
                    let right = gain(&n.right, best).max(0);
                    *best = (*best).max(n.val + left + right);
                    n.val + left.max(right)
                }
            }
        }
        let mut best = i32::MIN;
        gain(&root, &mut best);
        best
    }
}
