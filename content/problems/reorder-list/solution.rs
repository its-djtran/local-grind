impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Count the nodes, then cut the list after the end of the first half.
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref();
        }
        if len < 3 {
            return;
        }
        let mut mid = head.as_mut().unwrap();
        for _ in 0..(len - 1) / 2 {
            mid = mid.next.as_mut().unwrap();
        }
        let mut rest = mid.next.take();

        // Reverse the second half.
        let mut second = None;
        while let Some(mut node) = rest {
            rest = node.next.take();
            node.next = second;
            second = Some(node);
        }

        // Interleave the two halves.
        let mut first = head.as_mut();
        while let Some(mut node) = second {
            second = node.next.take();
            let f = first.unwrap();
            node.next = f.next.take();
            first = f.next.insert(node).next.as_mut();
        }
    }
}
