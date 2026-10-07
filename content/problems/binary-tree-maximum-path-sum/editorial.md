## Approach 1: Try Every Turning Point, Recomputing Branches

**Intuition.** Every path, viewed in the tree, climbs up to a single highest node and then (maybe)
goes back down the other side. So the best path is, for some node `v`, `v.val` plus the best
downward path starting at `v.left` plus the best downward path starting at `v.right` — where a
branch may be skipped (counted as `0`) if it is negative.

A direct implementation recomputes the best downward path from scratch for every candidate
turning point.

```python
class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def down(node):
            # best sum of a path that starts at `node` and only moves to children
            if node is None:
                return 0
            return node.val + max(0, down(node.left), down(node.right))

        best, stack = -inf, [root]
        while stack:
            node = stack.pop()
            if node is None:
                continue
            best = max(best, node.val + max(0, down(node.left)) + max(0, down(node.right)))
            stack += [node.left, node.right]
        return best
```

- **Time:** `O(n^2)` in the worst case — on a list-shaped tree, `down` walks the whole remaining
  chain for each of the `n` nodes. With `3 * 10^4` nodes that is far too slow.
- **Space:** `O(h)` for the recursion, where `h` is the tree height.

## Approach 2: One Post-Order Pass (Optimal)

**Intuition.** The repeated work in Approach 1 is the downward gain of each node, and that value
only depends on the children's gains. Computing it bottom-up, once per node, lets us evaluate
every turning point in the same pass.

**Algorithm.** Define `gain(node)` as the best sum of a downward path starting at `node`
(`0` for an empty tree).

1. Recursively compute `left = max(gain(node.left), 0)` and `right = max(gain(node.right), 0)`;
   clamping at `0` means "do not extend into a branch that only loses value".
2. The best path that turns at `node` is `node.val + left + right`; update a global `best`.
3. A path that continues upward through the parent can use only one side, so return
   `node.val + max(left, right)`.

Initialize `best` to negative infinity: if every value is negative, the answer is the largest
single value (for example `[-3]` gives `-3`), never `0`.

```python
class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        best = -inf

        def gain(node):
            nonlocal best
            if node is None:
                return 0
            left = max(gain(node.left), 0)
            right = max(gain(node.right), 0)
            best = max(best, node.val + left + right)
            return node.val + max(left, right)

        gain(root)
        return best
```

- **Time:** `O(n)` — each node is processed once.
- **Space:** `O(h)` for the recursion (`O(n)` for a degenerate tree). An iterative post-order
  traversal gives the same bounds without deep recursion.
