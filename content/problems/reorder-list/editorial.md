## Approach 1: Repeatedly Move the Tail

**Intuition.** Follow the definition literally: after each front node, cut the current last node
off the end and splice it in.

**Algorithm.** With `cur = head`, while `cur` has at least two nodes after it: walk to the
second-to-last node, detach the last node, insert it after `cur`, and move `cur` two nodes
forward.

```python
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        cur = head
        while cur and cur.next and cur.next.next:
            before_tail = cur
            while before_tail.next.next:
                before_tail = before_tail.next
            tail = before_tail.next
            before_tail.next = None
            tail.next = cur.next
            cur.next = tail
            cur = tail.next
```

- **Time:** `O(n^2)` — each splice walks to the end of the remaining list.
- **Space:** `O(1)`.

## Approach 2: Array of Nodes

**Intuition.** With every node in an array, the back of the list is just an index. Two indices
moving towards each other produce the required order.

**Algorithm.** Store the nodes in `nodes`. With `i = 0` and `j = n - 1`, link `nodes[i]` to
`nodes[j]` and `nodes[j]` to `nodes[i + 1]`, moving inwards until the indices meet. Finally,
terminate the list at the node where they met.

```python
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        nodes = []
        while head:
            nodes.append(head)
            head = head.next
        i, j = 0, len(nodes) - 1
        while i < j:
            nodes[i].next = nodes[j]
            i += 1
            if i == j:
                break
            nodes[j].next = nodes[i]
            j -= 1
        nodes[i].next = None
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the array.

## Approach 3: Middle, Reverse, Merge (Optimal)

**Intuition.** The answer interleaves the first half of the list with the second half read
backwards. Each piece is a standard linked-list routine, and none needs extra memory.

**Algorithm.**

1. **Find the middle** with slow and fast pointers. When `fast` can no longer take two steps,
   `slow` is the last node of the first half (for odd lengths the first half keeps the middle
   node).
2. **Reverse the second half**, starting at `slow.next`, and cut it off by setting
   `slow.next = None`.
3. **Merge**: take one node from the first half, then one from the reversed second half, and
   repeat. The second half is never longer than the first, so the loop ends when it runs out.

```python
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        slow = fast = head
        while fast.next and fast.next.next:
            slow = slow.next
            fast = fast.next.next

        prev, cur = None, slow.next
        slow.next = None
        while cur:
            cur.next, prev, cur = prev, cur, cur.next

        first, second = head, prev
        while second:
            first_next, second_next = first.next, second.next
            first.next = second
            second.next = first_next
            first, second = first_next, second_next
```

- **Time:** `O(n)` — three linear passes.
- **Space:** `O(1)`.
