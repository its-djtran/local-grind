## Approach 1: Descend Using Subtree Searches (Brute Force)

**Intuition.** Forget about ordering for a moment. The answer is the deepest node whose subtree
contains both targets. Starting at the root, if both targets are inside the left subtree, the
answer is down there too; the same goes for the right. Otherwise the current node is the answer.

```python
class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        def contains(node, target):
            stack = [node]
            while stack:
                cur = stack.pop()
                if cur is target:
                    return True
                if cur:
                    stack += [cur.left, cur.right]
            return False

        node = root
        while True:
            if contains(node.left, p) and contains(node.left, q):
                node = node.left
            elif contains(node.right, p) and contains(node.right, q):
                node = node.right
            else:
                return node
```

- **Time:** `O(n * h)`: every step down may search a whole subtree. For a list-shaped tree that
  is `O(n^2)`.
- **Space:** `O(n)` for the search stack.

## Approach 2: Recursion Guided by Values

**Intuition.** The search-tree ordering tells us where each target lives without searching. If
both `p.val` and `q.val` are smaller than the current value, both targets are in the left subtree,
so the answer is there as well. If both are larger, go right. In any other case the two targets
are on different sides (or one of them *is* the current node), so the current node is the
lowest point that still sees both.

```python
class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        if p.val < root.val and q.val < root.val:
            return self.lowestCommonAncestor(root.left, p, q)
        if p.val > root.val and q.val > root.val:
            return self.lowestCommonAncestor(root.right, p, q)
        return root
```

- **Time:** `O(h)`, where `h` is the height of the tree.
- **Space:** `O(h)` for the recursion stack.

## Approach 3: Iterative Walk (Optimal)

**Intuition.** The recursion above is a tail call, so it can be replaced by a loop that moves a
single pointer down the tree.

**Algorithm.**

1. Start at the root.
2. If both values are smaller than the current node's value, move left; if both are larger, move
   right.
3. Otherwise the current node is the split point: return it.

```python
class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        node = root
        while True:
            if p.val < node.val and q.val < node.val:
                node = node.left
            elif p.val > node.val and q.val > node.val:
                node = node.right
            else:
                return node
```

- **Time:** `O(h)`: `O(log n)` for a balanced tree and `O(n)` for a degenerate one.
- **Space:** `O(1)`.
