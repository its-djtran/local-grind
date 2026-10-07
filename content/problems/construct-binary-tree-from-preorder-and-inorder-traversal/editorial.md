## Approach 1: Recursive Splitting with Search and Slices

**Intuition.** In pre-order the root comes first. In in-order the root sits between its left
subtree and its right subtree. So locating `preorder[0]` inside `inorder` at position `k` tells us
the left subtree has `k` nodes. That splits both listings into a left part and a right part, and
each part describes a smaller tree in exactly the same format.

**Algorithm.**

1. If the listings are empty, return `None`.
2. Make a node from `preorder[0]`, and find `k = inorder.index(preorder[0])`.
3. Left subtree: `preorder[1:k+1]` with `inorder[:k]`. Right subtree: `preorder[k+1:]` with
   `inorder[k+1:]`.

```python
class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        if not preorder:
            return None
        root = TreeNode(preorder[0])
        k = inorder.index(preorder[0])
        root.left = self.buildTree(preorder[1:k + 1], inorder[:k])
        root.right = self.buildTree(preorder[k + 1:], inorder[k + 1:])
        return root
```

- **Time:** `O(n^2)` in the worst case — on a skewed tree each of the `n` calls searches and
  copies lists of length up to `n`.
- **Space:** `O(n^2)` in the worst case for the slices alive along the recursion path.

## Approach 2: Index Map and a Moving Pre-Order Pointer (Optimal)

**Intuition.** Both costs in Approach 1 are avoidable. A dictionary from value to in-order
position replaces the linear search. And slices are unnecessary: pre-order visits nodes in
exactly the order the recursion creates them (root, then the whole left subtree, then the whole
right subtree), so a single pointer that advances through `preorder` always points at the next
root to create. Each call only needs the in-order bounds `[lo, hi]` of its subtree.

**Algorithm.**

1. Build `pos[value] = index in inorder` and set a pointer `p = 0`.
2. `build(lo, hi)`: if `lo > hi`, return `None`. Otherwise take `preorder[p]` as the root and
   advance `p`. Let `k = pos[root value]`; build the left subtree from `(lo, k - 1)` **first**,
   then the right subtree from `(k + 1, hi)`.
3. Return `build(0, n - 1)`.

```python
class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        pos = {v: i for i, v in enumerate(inorder)}
        nxt = iter(preorder)

        def build(lo, hi):
            if lo > hi:
                return None
            node = TreeNode(next(nxt))
            k = pos[node.val]
            node.left = build(lo, k - 1)
            node.right = build(k + 1, hi)
            return node

        return build(0, len(inorder) - 1)
```

- **Time:** `O(n)` — every node is created once with `O(1)` work.
- **Space:** `O(n)` for the map, plus `O(h)` recursion (`O(n)` for a skewed tree).
