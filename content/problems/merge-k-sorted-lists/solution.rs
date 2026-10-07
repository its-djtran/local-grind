impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = lists
            .iter()
            .enumerate()
            .filter_map(|(i, head)| head.as_ref().map(|node| Reverse((node.val, i))))
            .collect();
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while let Some(Reverse((_, i))) = heap.pop() {
            let mut node = lists[i].take().unwrap();
            lists[i] = node.next.take();
            if let Some(next) = &lists[i] {
                heap.push(Reverse((next.val, i)));
            }
            tail = tail.next.insert(node);
        }
        dummy.next
    }
}
