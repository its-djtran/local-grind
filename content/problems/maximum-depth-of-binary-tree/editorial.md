## Approach 1: Recursive Depth-First Search (Optimal)

**Intuition.** A tree's depth is `1 + max(depth(left), depth(right))`, and an empty tree has
depth `0`. That definition is already an algorithm.

```python
class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right))
```

- **Time:** `O(n)` — every node is visited once.
- **Space:** `O(h)` for the recursion, where `h` is the tree height (`O(n)` for a degenerate,
  list-shaped tree).

## Approach 2: Level-Order Traversal

**Intuition.** Breadth-first search processes the tree one level at a time; the number of levels
is the depth. This avoids deep recursion on skewed trees.

```python
class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        depth, level = 0, [root] if root else []
        while level:
            depth += 1
            level = [c for node in level for c in (node.left, node.right) if c]
        return depth
```

- **Time:** `O(n)`.
- **Space:** `O(w)` where `w` is the widest level.
