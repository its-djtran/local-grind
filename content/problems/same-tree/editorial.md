## Approach 1: Compare Serializations

**Intuition.** A level-order listing that writes an explicit marker for every missing child
describes a tree completely: two trees are identical exactly when their listings are equal.
(Listing only the values is not enough, since `[1,2]` and `[1,null,2]` hold the same values.)

**Algorithm.** Serialize each tree breadth-first, emitting `None` for every absent child, and
compare the two lists.

```python
class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        def listing(root):
            out, queue = [], deque([root])
            while queue:
                node = queue.popleft()
                if node is None:
                    out.append(None)
                else:
                    out.append(node.val)
                    queue.append(node.left)
                    queue.append(node.right)
            return out

        return listing(p) == listing(q)
```

- **Time:** `O(n + m)` — both trees are always traversed completely, even when the roots differ.
- **Space:** `O(n + m)` for the two listings.

## Approach 2: Recursive Lockstep Comparison (Optimal)

**Intuition.** Compare the trees directly while walking them together. At each pair of
positions there are only three possibilities: both are empty (equal), exactly one is empty (not
equal), or both are nodes, in which case the values must match and both pairs of children must
match recursively.

**Algorithm.**

1. If `p` and `q` are both `None`, return `true`.
2. If only one of them is `None`, or their values differ, return `false`.
3. Return whether the left children are the same *and* the right children are the same.

Because of short-circuiting, the walk stops at the first mismatch.

```python
class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if p is None and q is None:
            return True
        if p is None or q is None or p.val != q.val:
            return False
        return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)
```

- **Time:** `O(min(n, m))` — each pair of positions is visited at most once, and the walk ends
  as soon as one tree runs out where the other does not.
- **Space:** `O(h)` for the recursion, where `h` is the height of the shorter walk (`O(n)` for a
  skewed tree). An explicit stack of `(p, q)` pairs gives the same bounds without recursion.
