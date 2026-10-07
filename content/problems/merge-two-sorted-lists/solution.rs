impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while let (Some(a), Some(b)) = (&list1, &list2) {
            let src = if a.val <= b.val { &mut list1 } else { &mut list2 };
            let mut node = src.take().unwrap();
            *src = node.next.take();
            tail = tail.next.insert(node);
        }
        tail.next = list1.or(list2);
        dummy.next
    }
}
