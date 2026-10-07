/**
 * @param {ListNode[]} lists
 * @return {ListNode}
 */
var mergeKLists = function(lists) {
    // Binary min-heap of list nodes keyed by val.
    const heap = lists.filter(node => node !== null);
    const less = (i, j) => heap[i].val < heap[j].val;
    const swap = (i, j) => { [heap[i], heap[j]] = [heap[j], heap[i]]; };
    const siftDown = (i) => {
        const n = heap.length;
        while (true) {
            let smallest = i;
            const l = 2 * i + 1, r = l + 1;
            if (l < n && less(l, smallest)) smallest = l;
            if (r < n && less(r, smallest)) smallest = r;
            if (smallest === i) return;
            swap(i, smallest);
            i = smallest;
        }
    };
    const siftUp = (i) => {
        while (i > 0) {
            const parent = (i - 1) >> 1;
            if (!less(i, parent)) return;
            swap(i, parent);
            i = parent;
        }
    };
    for (let i = (heap.length >> 1) - 1; i >= 0; i--) siftDown(i);

    const dummy = new ListNode();
    let tail = dummy;
    while (heap.length) {
        const node = heap[0];
        tail.next = node;
        tail = node;
        if (node.next) {
            heap[0] = node.next;
        } else {
            const last = heap.pop();
            if (heap.length) heap[0] = last;
        }
        if (heap.length) siftDown(0);
    }
    return dummy.next;
};
