## Approach 1: Recompute Heights at Every Node

**Intuition.** Look at any path from its highest node `v`. It goes down into `v`'s left subtree
as far as it can, and down into the right subtree as far as it can. With `height` meaning the
number of nodes on the longest downward path (so an empty tree has height `0`), the longest path
that bends at `v` has `height(v.left) + height(v.right)` edges. The diameter is the largest of
these values over all nodes.

The simplest implementation computes both heights from scratch at each node.

```python
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        def height(node):
            if node is None:
                return 0
            return 1 + max(height(node.left), height(node.right))

        best, stack = 0, [root]
        while stack:
            node = stack.pop()
            if node:
                best = max(best, height(node.left) + height(node.right))
                stack += [node.left, node.right]
        return best
```

- **Time:** `O(n^2)` in the worst case — on a list-shaped tree, each of the `n` nodes measures
  the whole chain below it. That is about `5 * 10^7` steps for `10^4` nodes.
- **Space:** `O(h)` for the recursion, where `h` is the tree height.

## Approach 2: Heights Bottom-Up in One Pass (Optimal)

**Intuition.** The only thing each node needs from its children is their heights, and the
children's heights are what a post-order traversal produces anyway. So compute heights
recursively and, while each node has both child heights in hand, update the best bend so far.

**Algorithm.**

1. `height(node)` returns `0` for `None`.
2. Otherwise compute `l = height(node.left)` and `r = height(node.right)`, set
   `best = max(best, l + r)`, and return `1 + max(l, r)`.
3. Call `height(root)` and return `best`.

```python
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        best = 0

        def height(node):
            nonlocal best
            if node is None:
                return 0
            left, right = height(node.left), height(node.right)
            best = max(best, left + right)
            return 1 + max(left, right)

        height(root)
        return best
```

- **Time:** `O(n)` — each node is visited once.
- **Space:** `O(h)` for the recursion (`O(n)` for a degenerate tree).
