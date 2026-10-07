use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut node = root;
        let mut k = k;
        loop {
            while let Some(n) = node {
                node = n.borrow().left.clone();
                stack.push(n);
            }
            let n = stack.pop().unwrap();
            let n = n.borrow();
            k -= 1;
            if k == 0 {
                return n.val;
            }
            node = n.right.clone();
        }
    }
}
