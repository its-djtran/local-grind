You are given the `root` of a binary search tree and two of its nodes, `p` and `q`. Return their **lowest common ancestor**: the deepest node in the tree that has both `p` and `q` as descendants.

A node is considered a descendant of itself, so if `p` lies in the subtree of `q`, the answer is `q` (and vice versa).

<!-- examples -->

**Constraints:**

- The tree contains between `2` and `10^5` nodes.
- `-10^9 <= Node.val <= 10^9`
- All `Node.val` are **unique**.
- `p != q`
- `p` and `q` are both present in the tree.
