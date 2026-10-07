## Approach 1: Top-Down Height Checks

**Intuition.** Translate the definition literally: the tree is balanced if the two subtree
heights at the root differ by at most one, and both subtrees are themselves balanced. Heights are
computed by a separate helper.

```python
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def height(node):
            if node is None:
                return 0
            return 1 + max(height(node.left), height(node.right))

        if root is None:
            return True
        return (abs(height(root.left) - height(root.right)) <= 1
                and self.isBalanced(root.left) and self.isBalanced(root.right))
```

- **Time:** `O(n log n)` when the tree is balanced (each node's height is recomputed once per
  ancestor, and there are `O(log n)` ancestors). In the worst case the repeated height
  computations cost `O(n^2)`, for example on a tree that is a long chain except for one small
  difference near the bottom.
- **Space:** `O(h)` for the recursion, where `h` is the tree height.

## Approach 2: Bottom-Up Heights with Early Exit (Optimal)

**Intuition.** Approach 1 asks for the same heights again and again. Instead, let one post-order
pass return each subtree's height, and check the balance condition right there, where both child
heights are already known. To report failure without a second return value, use `-1` as a
"this subtree is unbalanced" signal and pass it straight up.

**Algorithm.** `check(node)`:

1. An empty subtree returns `0`.
2. Compute `left = check(node.left)`; if it is `-1`, return `-1`. Do the same for the right.
3. If `|left - right| > 1`, return `-1`. Otherwise return `1 + max(left, right)`.

The tree is balanced exactly when `check(root) != -1`.

```python
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def check(node):
            if node is None:
                return 0
            left = check(node.left)
            if left < 0:
                return -1
            right = check(node.right)
            if right < 0 or abs(left - right) > 1:
                return -1
            return 1 + max(left, right)

        return check(root) >= 0
```

- **Time:** `O(n)` — each node is visited at most once.
- **Space:** `O(h)` for the recursion (`O(n)` for a degenerate tree).
