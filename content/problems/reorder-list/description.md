You are given the `head` of a singly linked list whose nodes, in order, are

`L0 -> L1 -> ... -> L(n-2) -> L(n-1)`

Rearrange the list so the nodes alternate between the front and the back:

`L0 -> L(n-1) -> L1 -> L(n-2) -> L2 -> L(n-3) -> ...`

The values stored in the nodes must stay as they are; only the `next` pointers may change. The function modifies the list **in place** and returns nothing. The list is judged by reading it from the original `head`.

<!-- examples -->

**Constraints:**

- The list contains between `1` and `5 * 10^4` nodes.
- `1 <= Node.val <= 1000`
