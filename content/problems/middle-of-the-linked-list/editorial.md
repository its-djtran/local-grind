## Approach 1: Count, Then Walk

**Intuition.** For a list of `n` nodes, the required node is at index `n // 2` (0-indexed). This
index gives the single middle for odd `n` and the second of the two middles for even `n`.

```python
class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        n, node = 0, head
        while node:
            n += 1
            node = node.next
        node = head
        for _ in range(n // 2):
            node = node.next
        return node
```

- **Time:** `O(n)` — two passes.
- **Space:** `O(1)`.

## Approach 2: Slow and Fast Pointers (Optimal)

**Intuition.** Move `slow` one node at a time and `fast` two nodes at a time. When `fast` reaches
the end, `slow` has covered half the distance and is at the middle.

**Algorithm.** Start both at `head`. While `fast` and `fast.next` exist, advance `slow` by one and
`fast` by two. Return `slow`.

With an even number of nodes, `fast` ends at `None` (it jumped past the last node), which leaves
`slow` on the second middle, as required. With an odd number, `fast` stops on the last node.

```python
class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        slow = fast = head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
        return slow
```

- **Time:** `O(n)` — a single pass.
- **Space:** `O(1)`.
