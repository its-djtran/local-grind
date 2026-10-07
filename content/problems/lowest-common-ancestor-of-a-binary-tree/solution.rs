use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root, p.as_ref().unwrap(), q.as_ref().unwrap())
    }

    fn dfs(root: &Node, p: &Rc<RefCell<TreeNode>>, q: &Rc<RefCell<TreeNode>>) -> Node {
        let node = root.as_ref()?;
        if Rc::ptr_eq(node, p) || Rc::ptr_eq(node, q) {
            return Some(Rc::clone(node));
        }
        let n = node.borrow();
        let left = Self::dfs(&n.left, p, q);
        let right = Self::dfs(&n.right, p, q);
        match (left, right) {
            (Some(_), Some(_)) => Some(Rc::clone(node)),
            (left, right) => left.or(right),
        }
    }
}
