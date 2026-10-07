## Approach 1: Heap-Indexed Array

**Intuition.** Store each node at the index it would have in an array-backed heap: the root is
index `0`, and the children of index `i` live at `2i + 1` and `2i + 2`. Empty slots hold a
marker. The array alone determines the tree, so writing it out is a valid encoding.

```python
class Codec:
    def serialize(self, root: Optional[TreeNode]) -> str:
        slots = {}
        stack = [(root, 0)]
        while stack:
            node, i = stack.pop()
            if node:
                slots[i] = node.val
                stack += [(node.left, 2 * i + 1), (node.right, 2 * i + 2)]
        size = max(slots) + 1 if slots else 0
        return ",".join(str(slots[i]) if i in slots else "#" for i in range(size))

    def deserialize(self, data: str) -> Optional[TreeNode]:
        tokens = data.split(",") if data else []
        nodes = [None if t == "#" else TreeNode(int(t)) for t in tokens]
        for i, node in enumerate(nodes):
            if node:
                if 2 * i + 1 < len(nodes):
                    node.left = nodes[2 * i + 1]
                if 2 * i + 2 < len(nodes):
                    node.right = nodes[2 * i + 2]
        return nodes[0] if nodes else None
```

- **Time / Space:** `O(2^h)` where `h` is the height. That is fine for balanced trees, but a
  list-shaped tree with `10^4` nodes would need an array with about `2^10000` slots, so this
  approach cannot handle the full constraints.

## Approach 2: Pre-Order with Null Markers

**Intuition.** Record only the slots that matter. Walk the tree in pre-order (node, left, right)
and write a `#` whenever a child is missing. While decoding, read tokens in the same order: the
first token is the root, then the entire left subtree follows, then the right subtree. The `#`
markers say exactly where each subtree stops, so the tree is rebuilt unambiguously.

```python
class Codec:
    def serialize(self, root: Optional[TreeNode]) -> str:
        out = []

        def walk(node):
            if node is None:
                out.append("#")
                return
            out.append(str(node.val))
            walk(node.left)
            walk(node.right)

        walk(root)
        return ",".join(out)

    def deserialize(self, data: str) -> Optional[TreeNode]:
        tokens = iter(data.split(","))

        def build():
            t = next(tokens)
            if t == "#":
                return None
            node = TreeNode(int(t))
            node.left = build()
            node.right = build()
            return node

        return build()
```

The comma delimiter matters: without it, `[1,23]` and `[12,3]` would both become `123`.

- **Time:** `O(n)` for each direction.
- **Space:** `O(n)` for the string, plus `O(h)` for the recursion.

## Approach 3: Level-Order with Null Markers (Optimal)

**Intuition.** Breadth-first order works just as well and needs no recursion. Encode the tree
level by level, writing `#` for each missing child. To decode, keep a queue of nodes still
waiting for their children; each node takes the next two tokens as its left and right child.

**Algorithm.**

1. `serialize`: run BFS from the root, pushing `None` children too. Write `#` for `None`,
   otherwise the value (and enqueue both children). Join with commas.
2. `deserialize`: if the first token is `#` (or the string is empty), return `None`. Create the
   root and enqueue it. For each dequeued node, read two tokens; each non-`#` token becomes a new
   child that is attached and enqueued.

```python
class Codec:
    def serialize(self, root: Optional[TreeNode]) -> str:
        out, queue = [], deque([root])
        while queue:
            node = queue.popleft()
            if node is None:
                out.append("#")
            else:
                out.append(str(node.val))
                queue.append(node.left)
                queue.append(node.right)
        return ",".join(out)

    def deserialize(self, data: str) -> Optional[TreeNode]:
        tokens = data.split(",")
        if not data or tokens[0] == "#":
            return None
        root = TreeNode(int(tokens[0]))
        queue, i = deque([root]), 1
        while queue:
            node = queue.popleft()
            if tokens[i] != "#":
                node.left = TreeNode(int(tokens[i]))
                queue.append(node.left)
            if tokens[i + 1] != "#":
                node.right = TreeNode(int(tokens[i + 1]))
                queue.append(node.right)
            i += 2
        return root
```

Each of the `n` nodes contributes one token and two child tokens are written per node, so the
string holds exactly `2n + 1` tokens.

- **Time:** `O(n)` for each direction.
- **Space:** `O(n)` for the string and the queue.
