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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(
            lo: usize,
            hi: usize, // exclusive
            preorder: &[i32],
            next: &mut usize,
            pos: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if lo >= hi {
                return None;
            }
            let val = preorder[*next];
            *next += 1;
            let k = pos[&val];
            let mut node = TreeNode::new(val);
            node.left = build(lo, k, preorder, next, pos);
            node.right = build(k + 1, hi, preorder, next, pos);
            Some(Rc::new(RefCell::new(node)))
        }
        let pos: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut next = 0;
        build(0, inorder.len(), &preorder, &mut next, &pos)
    }
}
