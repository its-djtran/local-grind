use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_b_s_t(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = Vec::new();
        let mut node = root;
        let mut prev = i64::MIN;
        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                node = n.borrow().left.clone();
                stack.push(n);
            }
            let n = stack.pop().unwrap();
            let n = n.borrow();
            if (n.val as i64) <= prev {
                return false;
            }
            prev = n.val as i64;
            node = n.right.clone();
        }
        true
    }
}
