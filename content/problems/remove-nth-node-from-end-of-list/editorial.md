## Approach 1: Measure the Length First

**Intuition.** In a list of length `L`, the `n`-th node from the end is node number `L - n`
(0-indexed) from the front. One pass finds `L`; a second pass stops just before that node and
unlinks it.

**Algorithm.**

1. Count the nodes to get `L`.
2. Put a `dummy` node in front of `head` so that removing the head needs no special case.
3. From `dummy`, step forward `L - n` times; the current node is now the predecessor of the
   target. Set `prev.next = prev.next.next`.
4. Return `dummy.next`.

```python
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        length, node = 0, head
        while node:
            length += 1
            node = node.next
        dummy = ListNode(0, head)
        prev = dummy
        for _ in range(length - n):
            prev = prev.next
        prev.next = prev.next.next
        return dummy.next
```

- **Time:** `O(L)` — two passes.
- **Space:** `O(1)`.

## Approach 2: Two Pointers, One Pass (Optimal)

**Intuition.** Keep two pointers exactly `n + 1` nodes apart. When the leading pointer runs off
the end of the list, the trailing pointer sits right before the node to delete.

**Algorithm.**

1. Create `dummy` in front of `head`; set `slow = fast = dummy`.
2. Advance `fast` by `n + 1` nodes.
3. Move both pointers one step at a time until `fast` is `None`.
4. Unlink `slow.next` and return `dummy.next`.

The dummy node matters when `n` equals the length: then `slow` never moves, and the removed node
is the original head.

```python
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        slow = fast = dummy
        for _ in range(n + 1):
            fast = fast.next
        while fast:
            slow = slow.next
            fast = fast.next
        slow.next = slow.next.next
        return dummy.next
```

- **Time:** `O(L)` — a single walk.
- **Space:** `O(1)`.
