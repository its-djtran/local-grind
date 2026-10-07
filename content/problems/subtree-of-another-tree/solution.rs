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
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Give every distinct subtree shape a unique integer id.
        fn ident(
            node: &Option<Rc<RefCell<TreeNode>>>,
            target: usize,
            ids: &mut HashMap<(usize, i32, usize), usize>,
            found: &mut bool,
        ) -> usize {
            let Some(n) = node else { return 0 };
            let n = n.borrow();
            let key = (ident(&n.left, target, ids, found), n.val, ident(&n.right, target, ids, found));
            let next = ids.len() + 1;
            let id = *ids.entry(key).or_insert(next);
            if id == target {
                *found = true;
            }
            id
        }
        let mut ids = HashMap::new();
        let mut found = false;
        let target = ident(&sub_root, usize::MAX, &mut ids, &mut found);
        ident(&root, target, &mut ids, &mut found);
        found
    }
}
