## Approach 1: Collect and Sort

**Intuition.** Ignore the search-tree property entirely: gather every value, sort, and index.

```python
class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        values, stack = [], [root]
        while stack:
            node = stack.pop()
            if node:
                values.append(node.val)
                stack += [node.left, node.right]
        return sorted(values)[k - 1]
```

- **Time:** `O(n log n)` for the sort.
- **Space:** `O(n)` for the list of values.

## Approach 2: Recursive In-Order Traversal

**Intuition.** In a binary search tree, an in-order traversal (left subtree, node, right subtree)
already produces the values in ascending order, so no sorting is needed.

```python
class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        def inorder(node):
            if node is None:
                return []
            return inorder(node.left) + [node.val] + inorder(node.right)

        return inorder(root)[k - 1]
```

- **Time:** `O(n)` visits (list concatenation adds overhead, but no sorting).
- **Space:** `O(n)` for the full list.

## Approach 3: Iterative In-Order with Early Exit (Optimal)

**Intuition.** We do not need the whole sorted sequence, only its first `k` entries. An explicit
stack lets us run the in-order walk step by step and stop the moment the `k`-th value appears.

**Algorithm.**

1. Starting from the root, push nodes while moving left; the top of the stack is then the
   smallest value not yet visited.
2. Pop a node and decrement `k`. If `k` reaches `0`, that node holds the answer.
3. Otherwise move to the popped node's right child and repeat.

```python
class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        stack, node = [], root
        while True:
            while node:
                stack.append(node)
                node = node.left
            node = stack.pop()
            k -= 1
            if k == 0:
                return node.val
            node = node.right
```

- **Time:** `O(h + k)`, where `h` is the tree height: we descend one left spine and then visit
  `k` nodes.
- **Space:** `O(h)` for the stack.

**Follow-up.** If the tree changes frequently, augment each node with the size of its subtree
(maintained during insertions and deletions, ideally in a self-balancing tree). To find the
`k`-th smallest from a node, let `L` be the size of its left subtree: if `k <= L` go left, if
`k == L + 1` the node is the answer, otherwise go right looking for the `(k - L - 1)`-th
smallest. Each query and update then costs `O(h)`, which is `O(log n)` in a balanced tree.
