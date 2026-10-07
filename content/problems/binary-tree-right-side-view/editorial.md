## Approach 1: Depth-First Search, Right Child First

**Intuition.** Walk the tree depth first but always explore the right child before the left one.
Then, for every depth, the first node we reach is the rightmost node on that level. If the answer
list is exactly as long as the current depth, this node is the first we have seen at that depth,
so it is visible.

```python
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        view = []

        def dfs(node, depth):
            if node is None:
                return
            if depth == len(view):
                view.append(node.val)
            dfs(node.right, depth + 1)
            dfs(node.left, depth + 1)

        dfs(root, 0)
        return view
```

- **Time:** `O(n)` — every node is visited once.
- **Space:** `O(h)` for the recursion stack, where `h` is the tree height.

## Approach 2: Level-Order Traversal (Optimal)

**Intuition.** Breadth-first search processes the tree one level at a time, from left to right.
The visible node of a level is simply the last node in that level.

**Algorithm.**

1. Start with `level = [root]` (or an empty list for an empty tree).
2. While the level is non-empty, append the value of its last node to the answer, then build the
   next level from the children of the current one, left child before right child.

```python
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        view, level = [], [root] if root else []
        while level:
            view.append(level[-1].val)
            level = [c for node in level for c in (node.left, node.right) if c]
        return view
```

- **Time:** `O(n)`.
- **Space:** `O(w)`, where `w` is the width of the widest level.
