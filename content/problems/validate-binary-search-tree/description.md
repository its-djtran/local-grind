You are given the `root` of a binary tree. Decide whether it is a **valid binary search tree** and return `true` if it is, `false` otherwise.

A binary search tree is valid when all of the following hold:

- Every value in a node's left subtree is **strictly less** than the node's own value.
- Every value in a node's right subtree is **strictly greater** than the node's own value.
- Both the left and the right subtree are themselves valid binary search trees.

Because the comparisons are strict, a tree containing two equal values can never be valid.

<!-- examples -->

**Constraints:**

- The tree contains between `1` and `10^4` nodes.
- `-2^31 <= Node.val <= 2^31 - 1`
