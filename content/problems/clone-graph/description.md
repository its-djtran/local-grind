You are handed a reference to one node of a **connected, undirected** graph. Build and return a **deep copy** of the whole graph: a new set of nodes with the same values and the same edges, where no node of the copy is a node of the original.

Each node stores an integer `val` and a list `neighbors` of the nodes it is connected to:

```python
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
```

**How tests are written.** For readability the graph is shown as an adjacency list. Node values are `1, 2, ..., n`, and the `i`-th entry of the list (1-indexed) holds the values of node `i`'s neighbors. The node passed to your function is always the one with value `1`. Your returned node is serialized the same way, so it must be the copy of node `1`.

If the graph is empty, the input is `null` and you should return `null`.

<!-- examples -->

**Constraints:**

- The graph has between `0` and `100` nodes.
- `1 <= Node.val <= 100`
- Every `Node.val` is unique.
- There are no self-loops and no repeated edges.
- The graph is connected, so every node can be reached from the given node.
