A graph on `n` nodes is a tree when it is **connected** and **acyclic**. A useful fact ties these
together: a tree on `n` nodes has exactly `n - 1` edges, and any graph with exactly `n - 1` edges
is connected if and only if it is acyclic. So after checking the edge count, only one of the two
properties needs testing.

## Approach 1: Reject Each Edge That Closes a Cycle (Repeated Search)

**Intuition.** Add edges one at a time. Before adding `[a, b]`, search the graph built so far to
see whether `a` can already reach `b`; if so this edge closes a cycle. At the end, make sure
everything is connected.

```python
class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        graph = [[] for _ in range(n)]
        for a, b in edges:
            seen, stack = {a}, [a]
            while stack:
                v = stack.pop()
                if v == b:
                    return False
                for w in graph[v]:
                    if w not in seen:
                        seen.add(w)
                        stack.append(w)
            graph[a].append(b)
            graph[b].append(a)
        seen, stack = {0}, [0]
        while stack:
            for w in graph[stack.pop()]:
                if w not in seen:
                    seen.add(w)
                    stack.append(w)
        return len(seen) == n
```

- **Time:** `O(E * (V + E))` — a full search per edge.
- **Space:** `O(V + E)`.

## Approach 2: Edge Count + One Traversal

**Intuition.** If `len(edges) != n - 1` the answer is `false`. Otherwise it suffices to check
connectivity: one BFS from node `0` must reach all `n` nodes.

```python
class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) != n - 1:
            return False
        graph = [[] for _ in range(n)]
        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)
        seen, queue = {0}, deque([0])
        while queue:
            for w in graph[queue.popleft()]:
                if w not in seen:
                    seen.add(w)
                    queue.append(w)
        return len(seen) == n
```

- **Time:** `O(V + E)`.
- **Space:** `O(V + E)` for the adjacency list.

## Approach 3: Edge Count + Union-Find (Optimal)

**Intuition.** Union-find answers "are `a` and `b` already connected?" in near-constant time.
With exactly `n - 1` edges, the graph is a tree precisely when no edge joins two nodes that are
already in the same component.

**Algorithm.**

1. Return `false` if `len(edges) != n - 1`.
2. For each edge, find the roots of both endpoints. Equal roots mean a cycle: return `false`.
   Otherwise union them.
3. Return `true`.

```python
class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) != n - 1:
            return False
        parent = list(range(n))

        def find(x):
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        for a, b in edges:
            ra, rb = find(a), find(b)
            if ra == rb:
                return False
            parent[ra] = rb
        return True
```

- **Time:** `O(V * α(V))`, effectively linear (at most `n - 1` edges are ever processed).
- **Space:** `O(V)` for the parent array.
