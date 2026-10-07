impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Two pointers can't alias a Box list in safe Rust, so measure the length first
        // and then walk to the node before the one to remove: still O(n) time, O(1) space.
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref();
        }
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;
        for _ in 0..len - n {
            prev = prev.next.as_mut().unwrap();
        }
        let removed = prev.next.take().unwrap();
        prev.next = removed.next;
        dummy.next
    }
}
