## Approach 1: Measure the Height from Every Root

**Intuition.** The height of the tree rooted at `v` is the largest BFS distance from `v`. Compute
it for every `v` and keep the roots that achieve the minimum.

```python
class Solution:
    def findMinHeightTrees(self, n: int, edges: List[List[int]]) -> List[int]:
        graph = [[] for _ in range(n)]
        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)

        def height(root):
            dist = [-1] * n
            dist[root] = 0
            queue = deque([root])
            while queue:
                v = queue.popleft()
                for w in graph[v]:
                    if dist[w] == -1:
                        dist[w] = dist[v] + 1
                        queue.append(w)
            return max(dist)

        heights = [height(v) for v in range(n)]
        best = min(heights)
        return [v for v in range(n) if heights[v] == best]
```

- **Time:** `O(n^2)` — a full BFS for each of the `n` candidate roots. With `n = 2 * 10^4`
  that is hundreds of millions of steps.
- **Space:** `O(n)`.

## Approach 2: Peel Off Leaves (Optimal)

**Intuition.** The height from root `v` is the distance from `v` to the farthest node, and the
farthest node is always a leaf. Moving the root one step inward from a leaf can only bring it
closer to the far side of the tree, so leaves are never better roots than their neighbors (once
`n > 2`). Remove all current leaves at once: the remaining tree has the same best roots, because
every distance to a farthest leaf dropped by exactly one. Repeat until one or two nodes remain;
those are the centers of the tree. (A tree has either one center or two adjacent centers: the
middle of its longest path.)

**Algorithm.**

1. If `n <= 2`, every node is an answer.
2. Build adjacency sets and collect all leaves (degree `1`).
3. While more than two nodes remain: remove the whole current layer of leaves; any neighbor whose
   degree drops to `1` joins the next layer.
4. Return the last layer.

This is topological sort on an undirected tree, processing nodes by degree.

```python
class Solution:
    def findMinHeightTrees(self, n: int, edges: List[List[int]]) -> List[int]:
        if n <= 2:
            return list(range(n))
        graph = [[] for _ in range(n)]
        degree = [0] * n
        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)
            degree[a] += 1
            degree[b] += 1
        leaves = [v for v in range(n) if degree[v] == 1]
        remaining = n
        while remaining > 2:
            remaining -= len(leaves)
            nxt = []
            for leaf in leaves:
                for w in graph[leaf]:
                    degree[w] -= 1
                    if degree[w] == 1:
                        nxt.append(w)
            leaves = nxt
        return leaves
```

- **Time:** `O(n)` — every node is removed once and every edge is examined twice.
- **Space:** `O(n)` for the adjacency lists and degree array.

An alternative linear solution finds a diameter with two BFS passes (farthest node from any
node, then farthest node from that one) and returns the middle one or two nodes of that path.
