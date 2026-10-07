You are given the `root` of a binary tree (not necessarily a search tree) and two of its nodes, `p` and `q`. Return their **lowest common ancestor**: the node farthest from the root that has both `p` and `q` in its subtree.

Every node is part of its own subtree, so when one of the two nodes lies beneath the other, the upper one is the answer.

<!-- examples -->

**Constraints:**

- The tree contains between `2` and `10^5` nodes.
- `-10^9 <= Node.val <= 10^9`
- All `Node.val` are **unique**.
- `p != q`
- `p` and `q` are both present in the tree.
