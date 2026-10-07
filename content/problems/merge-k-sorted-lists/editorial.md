Throughout, `k` is the number of lists and `N` the total number of nodes.

## Approach 1: Merge One List at a Time

**Intuition.** Reuse the two-list merge: start with an empty result and merge each list into it
in turn.

```python
class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        def merge(a, b):
            dummy = tail = ListNode()
            while a and b:
                if a.val <= b.val:
                    tail.next, a = a, a.next
                else:
                    tail.next, b = b, b.next
                tail = tail.next
            tail.next = a or b
            return dummy.next

        result = None
        for head in lists:
            result = merge(result, head)
        return result
```

- **Time:** `O(k * N)` — the growing result is walked again for every list. With `10^4` lists of
  one node each, that is about `5 * 10^7` steps.
- **Space:** `O(1)`.

## Approach 2: Collect and Sort

**Intuition.** Ignore the existing order: gather every node, sort by value, and relink.

```python
class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        nodes = []
        for head in lists:
            while head:
                nodes.append(head)
                head = head.next
        nodes.sort(key=lambda node: node.val)
        dummy = tail = ListNode()
        for node in nodes:
            tail.next = node
            tail = node
        tail.next = None
        return dummy.next
```

- **Time:** `O(N log N)`.
- **Space:** `O(N)` for the node array.

This is fast in practice but does not use the fact that each list is already sorted.

## Approach 3: Min-Heap of List Heads (Optimal)

**Intuition.** The next node of the answer is the smallest of the `k` current heads. A min-heap
holding one head per list returns that minimum in `O(log k)`, and after popping a node, only its
successor needs to enter the heap.

**Algorithm.**

1. Push `(head.val, i, head)` for every non-empty list `i`. The index `i` breaks ties so Python
   never tries to compare two `ListNode` objects.
2. While the heap is non-empty: pop the smallest entry, append its node to the result, and if
   the node has a successor, push `(next.val, i, next)`.
3. Return the result after the dummy head.

```python
class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        heap = [(head.val, i, head) for i, head in enumerate(lists) if head]
        heapq.heapify(heap)
        dummy = tail = ListNode()
        while heap:
            _, i, node = heapq.heappop(heap)
            tail.next = node
            tail = node
            if node.next:
                heapq.heappush(heap, (node.next.val, i, node.next))
        return dummy.next
```

- **Time:** `O(N log k)` — every node is pushed and popped once on a heap of at most `k` entries.
- **Space:** `O(k)` for the heap.

Merging the lists in pairs (round 1 merges lists `0+1`, `2+3`, ...; round 2 merges the results,
and so on) achieves the same `O(N log k)` bound with `O(1)` extra space, since each of the
`log k` rounds touches every node once.
