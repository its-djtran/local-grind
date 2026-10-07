use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        let mut node = root;
        while let Some(n) = node {
            let v = n.borrow().val;
            if p < v && q < v {
                node = n.borrow().left.clone();
            } else if p > v && q > v {
                node = n.borrow().right.clone();
            } else {
                return Some(n);
            }
        }
        None
    }
}
