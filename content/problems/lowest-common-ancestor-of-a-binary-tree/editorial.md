## Approach 1: Descend Using Subtree Searches (Brute Force)

**Intuition.** The answer is the deepest node whose subtree holds both targets. Start at the root,
which certainly holds both. If one child's subtree also holds both, the answer is deeper on that
side; otherwise the current node is the answer.

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

- **Time:** `O(n * h)`: each step down may search an entire subtree, which is `O(n^2)` when the
  tree is one long path.
- **Space:** `O(n)` for the search stack.

## Approach 2: Parent Pointers and an Ancestor Set

**Intuition.** If every node knew its parent, we could walk upward from `p`, remembering every
ancestor, and then walk upward from `q` until we step on a remembered node. The first such node
is the lowest common ancestor.

**Algorithm.**

1. Traverse the tree (iteratively) and record `parent[child] = node` until both `p` and `q` have
   been seen.
2. Collect all ancestors of `p` (including `p`) into a set.
3. Move `q` upward until it is in the set; return it.

```python
class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        parent = {root: None}
        stack = [root]
        while p not in parent or q not in parent:
            node = stack.pop()
            for child in (node.left, node.right):
                if child:
                    parent[child] = node
                    stack.append(child)
        ancestors = set()
        while p:
            ancestors.add(p)
            p = parent[p]
        while q not in ancestors:
            q = parent[q]
        return q
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the parent map.

## Approach 3: Single Recursive Pass (Optimal)

**Intuition.** Let `dfs(node)` return `p` or `q` if it finds one of them in the subtree of
`node`, the lowest common ancestor if it finds both, and `None` if it finds neither.

- If `node` is empty, or is `p` or `q` itself, return `node`. When `node` is a target we can stop:
  if the other target is below it, `node` is the answer; if not, the caller needs exactly this
  node as evidence.
- Otherwise ask both children. If both return something, `p` and `q` are on opposite sides, so
  `node` is where their paths meet. If only one side returns something, pass it upward.

**Algorithm.**

```python
class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        if root is None or root is p or root is q:
            return root
        left = self.lowestCommonAncestor(root.left, p, q)
        right = self.lowestCommonAncestor(root.right, p, q)
        if left and right:
            return root
        return left or right
```

- **Time:** `O(n)` — each node is visited at most once.
- **Space:** `O(h)` for the recursion stack, where `h` is the height of the tree.
