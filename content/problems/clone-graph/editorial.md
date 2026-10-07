The difficulty is that the graph can contain cycles. A naive recursive copy (`copy(node) =
Node(val, [copy(n) for n in neighbors])`) would bounce between two neighbors forever. The fix in
both approaches below is a map from each original node to its clone: it tells us whether a node
has already been copied and, if so, which object to link to.

## Approach 1: Depth-First Search with a Clone Map

**Intuition.** Copying a node means creating a fresh node with the same value and then wiring it
to the copies of its neighbors. If we register the fresh node in the map *before* recursing into
the neighbors, any cycle that leads back to it finds the existing copy instead of recursing
again.

**Algorithm.**

1. If the input is `None`, return `None`.
2. `clone(node)`: if `node` is in the map, return its copy. Otherwise create `copy = Node(node.val)`,
   store `map[node] = copy`, then set `copy.neighbors = [clone(n) for n in node.neighbors]` and
   return `copy`.

```python
class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        copies = {}

        def clone(cur):
            if cur in copies:
                return copies[cur]
            copy = Node(cur.val)
            copies[cur] = copy
            copy.neighbors = [clone(n) for n in cur.neighbors]
            return copy

        return clone(node) if node else None
```

- **Time:** `O(V + E)` — each node is created once and each adjacency entry is followed once.
- **Space:** `O(V)` for the map plus `O(V)` recursion depth in the worst case (a long path).

## Approach 2: Breadth-First Search (Optimal, Iterative)

**Intuition.** The same idea works level by level with a queue, which avoids recursion depth
concerns entirely. A node is cloned the moment it is first discovered; when it is later popped
from the queue, we fill in its clone's neighbor list.

**Algorithm.**

1. If the input is `None`, return `None`.
2. Create the clone of the start node, put it in the map, and push the start node on a queue.
3. While the queue is not empty, pop `cur`. For each neighbor `n` of `cur`:
   - If `n` has no clone yet, create one, record it, and enqueue `n`.
   - Append `map[n]` to `map[cur].neighbors`.
4. Return `map[start]`.

Because each original node is popped exactly once, every clone's neighbor list is filled exactly
once and in the same order as the original.

```python
class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if node is None:
            return None
        copies = {node: Node(node.val)}
        queue = deque([node])
        while queue:
            cur = queue.popleft()
            for n in cur.neighbors:
                if n not in copies:
                    copies[n] = Node(n.val)
                    queue.append(n)
                copies[cur].neighbors.append(copies[n])
        return copies[node]
```

- **Time:** `O(V + E)`.
- **Space:** `O(V)` for the map and the queue.
