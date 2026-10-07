You are given `head`, the first node of a singly linked list. Decide whether the list contains a **cycle**: some node that can be reached again by repeatedly following `next` pointers. Return `true` if such a loop exists and `false` otherwise.

In the examples, the list is shown together with `pos`, the index of the node that the tail's `next` pointer links back to (`-1` means the tail points to `None`). `pos` is only used to build the test input; your function receives just `head`.

<!-- examples -->

**Constraints:**

- The list contains between `0` and `10^4` nodes.
- `-10^5 <= Node.val <= 10^5`
- `pos` is `-1` or a valid index into the list.

**Follow-up:** Can you solve it using only `O(1)` extra memory?
