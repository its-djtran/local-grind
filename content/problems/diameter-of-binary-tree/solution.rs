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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
            let Some(n) = node else { return 0 };
            let n = n.borrow();
            let left = height(&n.left, best);
            let right = height(&n.right, best);
            *best = (*best).max(left + right);
            1 + left.max(right)
        }
        let mut best = 0;
        height(&root, &mut best);
        best
    }
}
