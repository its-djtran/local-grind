## Approach 1: One Depth-Limited Search per Level

**Intuition.** The values on level `d` are exactly the nodes reached after `d` steps from the
root, visited left before right. A depth-first search that stops at depth `d` collects them in
the correct order, so we can run one such search for every level until a level comes back empty.

```python
class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        def collect(node, depth, out):
            if node is None:
                return
            if depth == 0:
                out.append(node.val)
                return
            collect(node.left, depth - 1, out)
            collect(node.right, depth - 1, out)

        levels, d = [], 0
        while True:
            level = []
            collect(root, d, level)
            if not level:
                return levels
            levels.append(level)
            d += 1
```

- **Time:** `O(n * h)` where `h` is the height — every level re-walks all the nodes above it.
  For a list-shaped tree that is `O(n^2)`.
- **Space:** `O(h)` for the recursion, plus the output.

## Approach 2: Depth-First Search with a Depth Index

**Intuition.** Do a single pre-order traversal (node, then left, then right) and carry the depth.
Within any one depth, pre-order visits nodes left to right, so appending each value to
`levels[depth]` produces correctly ordered levels.

```python
class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        levels = []

        def visit(node, depth):
            if node is None:
                return
            if depth == len(levels):
                levels.append([])
            levels[depth].append(node.val)
            visit(node.left, depth + 1)
            visit(node.right, depth + 1)

        visit(root, 0)
        return levels
```

- **Time:** `O(n)`.
- **Space:** `O(h)` for the recursion, plus the output.

## Approach 3: Breadth-First Search by Levels (Optimal)

**Intuition.** A FIFO queue naturally hands out nodes in level order. To know where one level
ends and the next begins, record the queue length at the start of each level: exactly that many
nodes belong to the current level, and every child pushed while processing them belongs to the
next one.

**Algorithm.**

1. If `root` is `None`, return `[]`. Otherwise start with a queue holding `root`.
2. While the queue is non-empty: pop `len(queue)` nodes, appending each value to a new list and
   pushing its non-null children (left, then right). Append that list to the answer.

```python
class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []
        levels, queue = [], deque([root])
        while queue:
            level = []
            for _ in range(len(queue)):
                node = queue.popleft()
                level.append(node.val)
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
            levels.append(level)
        return levels
```

- **Time:** `O(n)` — every node is enqueued and dequeued once.
- **Space:** `O(w)` for the queue, where `w` is the widest level (up to about `n / 2`).
