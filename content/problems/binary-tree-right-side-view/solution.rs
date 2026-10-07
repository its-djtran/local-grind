use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut view = Vec::new();
        let mut level: Vec<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        while let Some(last) = level.last() {
            view.push(last.borrow().val);
            let mut next = Vec::new();
            for node in &level {
                let node = node.borrow();
                next.extend(node.left.clone());
                next.extend(node.right.clone());
            }
            level = next;
        }
        view
    }
}
