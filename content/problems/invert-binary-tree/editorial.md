## Approach 1: Rebuild a Mirrored Copy

**Intuition.** The mirror of a tree has the same root value, with the mirror of the original
right subtree on its left and the mirror of the original left subtree on its right. Building
fresh nodes with that rule leaves the input untouched.

```python
class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root is None:
            return None
        return TreeNode(root.val, self.invertTree(root.right), self.invertTree(root.left))
```

- **Time:** `O(n)` — one new node per original node.
- **Space:** `O(n)` for the copy, plus `O(h)` recursion where `h` is the tree height.

## Approach 2: Swap Children In Place (Optimal)

**Intuition.** No new nodes are needed: visiting every node once and exchanging its two child
pointers produces the mirror. The order of visits does not matter, so recursion, an explicit
stack, or a queue all work.

**Algorithm.**

1. If `root` is `None`, return `None`.
2. Swap `root.left` and `root.right` in a single tuple assignment, mirroring each child as it
   moves.
3. Return `root`.

A common slip is writing `root.left = invert(root.right)` followed by
`root.right = invert(root.left)`: by the second line, `root.left` already holds the new subtree,
so the original left subtree is lost. A tuple assignment evaluates both sides first.

```python
class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root is None:
            return None
        root.left, root.right = self.invertTree(root.right), self.invertTree(root.left)
        return root
```

- **Time:** `O(n)` — each node is visited once.
- **Space:** `O(h)` for the recursion (`O(n)` for a skewed tree, `O(log n)` for a balanced one).
