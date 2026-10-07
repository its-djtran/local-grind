## Approach 1: Recursion

**Intuition.** If the rest of the list (`head.next` onward) were already reversed, its last node
would be `head.next`. Pointing `head.next.next` back at `head` and clearing `head.next` finishes
the job.

```python
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return head
        new_head = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        return new_head
```

- **Time:** `O(n)` — one call per node.
- **Space:** `O(n)` for the call stack.

## Approach 2: Iterative Pointer Reversal (Optimal)

**Intuition.** Walk the list once, flipping each `next` pointer to point backwards. Before
flipping, save the node that comes next so the walk can continue.

**Algorithm.**

1. Start with `prev = None` and `cur = head`.
2. While `cur` is not `None`: save `nxt = cur.next`, set `cur.next = prev`, then advance
   `prev = cur` and `cur = nxt`.
3. `prev` is the new head.

```python
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        prev, cur = None, head
        while cur:
            nxt = cur.next
            cur.next = prev
            prev, cur = cur, nxt
        return prev
```

- **Time:** `O(n)`.
- **Space:** `O(1)` — only a few pointers.
