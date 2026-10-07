impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Run a borrowed fast pointer two nodes at a time to count the slow pointer's
        // steps, then advance the owned head that far (no cloning, O(1) extra space).
        let mut steps = 0;
        let mut fast = head.as_ref();
        while let Some(node) = fast.and_then(|f| f.next.as_ref()) {
            fast = node.next.as_ref();
            steps += 1;
        }
        for _ in 0..steps {
            head = head.unwrap().next;
        }
        head
    }
}
