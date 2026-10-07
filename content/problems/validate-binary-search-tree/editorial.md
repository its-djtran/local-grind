## Approach 1: Compare Against Whole Subtrees (Brute Force)

**Intuition.** The definition talks about *entire* subtrees, so check it literally: a node is fine
when the largest value on its left is smaller than it, the smallest value on its right is larger
than it, and both children pass the same test.

**Algorithm.** Write helpers that return the minimum and maximum of a subtree by visiting all of
its nodes, then apply the definition at every node.

```python
class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def lo(node):
            if node is None:
                return math.inf
            return min(node.val, lo(node.left), lo(node.right))

        def hi(node):
            if node is None:
                return -math.inf
            return max(node.val, hi(node.left), hi(node.right))

        def ok(node):
            if node is None:
                return True
            if hi(node.left) >= node.val or lo(node.right) <= node.val:
                return False
            return ok(node.left) and ok(node.right)

        return ok(root)
```

- **Time:** `O(n^2)` in the worst case. Every node scans its whole subtree; on a list-shaped tree
  that is `n + (n - 1) + ... + 1` visits.
- **Space:** `O(h)` for recursion, where `h` is the tree height.

## Approach 2: Pass Down Valid Ranges

**Intuition.** Instead of looking down, carry constraints down. The root may hold any value. When
we move to a left child, every value below must also be smaller than the parent, so the upper
bound tightens to the parent's value; moving right tightens the lower bound. A node is valid
exactly when its value sits strictly inside the interval it inherited.

**Algorithm.**

1. Call `check(node, low, high)` starting with `(root, -inf, inf)`.
2. An empty node is valid. Otherwise require `low < node.val < high`.
3. Recurse with `(node.left, low, node.val)` and `(node.right, node.val, high)`.

```python
class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def check(node, low, high):
            if node is None:
                return True
            if not (low < node.val < high):
                return False
            return check(node.left, low, node.val) and check(node.right, node.val, high)

        return check(root, -math.inf, math.inf)
```

Using infinities (rather than the 32-bit limits) for the initial bounds matters: a node holding
`2^31 - 1` or `-2^31` is perfectly legal.

- **Time:** `O(n)` — each node is checked once.
- **Space:** `O(h)` for the recursion.

## Approach 3: Iterative In-Order Traversal (Optimal)

**Intuition.** An in-order traversal (left, node, right) of a binary search tree lists its values
in sorted order. So the tree is valid if and only if that sequence is *strictly increasing*. We do
not need to store the sequence; comparing each value with the one visited just before suffices.

**Algorithm.**

1. Walk the tree in order with an explicit stack: push the left spine, pop a node, visit it, then
   continue with its right child.
2. Keep `prev`, the last visited value. If the current value is `<= prev`, return `false`.
3. If the walk completes, return `true`.

```python
class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        stack, node, prev = [], root, -math.inf
        while stack or node:
            while node:
                stack.append(node)
                node = node.left
            node = stack.pop()
            if node.val <= prev:
                return False
            prev = node.val
            node = node.right
        return True
```

The explicit stack also avoids deep recursion on a tree that degenerates into a long path.

- **Time:** `O(n)` — each node is pushed and popped once, and we stop at the first violation.
- **Space:** `O(h)` for the stack.
