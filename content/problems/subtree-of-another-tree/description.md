You are given the roots of two binary trees, `root` and `subRoot`. Return `true` if some node of `root` is the top of a subtree that is **identical** to `subRoot` (same shape and same values), and `false` otherwise.

The subtree of a node consists of that node together with **all** of its descendants. In particular, the whole of `root` counts as one of its own subtrees.

<!-- examples -->

**Constraints:**

- `root` contains between `1` and `2000` nodes.
- `subRoot` contains between `1` and `1000` nodes.
- `-10^4 <= root.val <= 10^4`
- `-10^4 <= subRoot.val <= 10^4`
