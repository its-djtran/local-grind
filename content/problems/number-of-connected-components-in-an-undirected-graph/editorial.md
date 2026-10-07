## Approach 1: Relabel on Every Edge ("Quick-Find")

**Intuition.** Give every node a component label, initially its own index. An edge `[a, b]`
between different labels merges the two components, which we can do by rewriting every
occurrence of `b`'s label with `a`'s label. The answer is the number of distinct labels left.

```python
class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        label = list(range(n))
        for a, b in edges:
            la, lb = label[a], label[b]
            if la != lb:
                label = [la if x == lb else x for x in label]
        return len(set(label))
```

- **Time:** `O(E * V)` — each edge rewrites the whole label array.
- **Space:** `O(V)`.

## Approach 2: Depth-First Search

**Intuition.** Build an adjacency list. Each time we start a traversal from a node that has not
been visited, we have found a new component, and the traversal marks all of it.

```python
class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        graph = [[] for _ in range(n)]
        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)
        seen = [False] * n
        count = 0
        for s in range(n):
            if seen[s]:
                continue
            count += 1
            seen[s] = True
            stack = [s]
            while stack:
                for w in graph[stack.pop()]:
                    if not seen[w]:
                        seen[w] = True
                        stack.append(w)
        return count
```

- **Time:** `O(V + E)`.
- **Space:** `O(V + E)` for the adjacency list.

## Approach 3: Union-Find (Optimal)

**Intuition.** Start with `n` components. Each edge whose endpoints have different roots merges
two components, lowering the count by one; an edge inside a component changes nothing. Path
compression (here, path halving) plus union by size keeps every operation nearly constant time,
and no adjacency list is needed.

```python
class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        parent = list(range(n))
        size = [1] * n

        def find(x):
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        count = n
        for a, b in edges:
            ra, rb = find(a), find(b)
            if ra != rb:
                if size[ra] < size[rb]:
                    ra, rb = rb, ra
                parent[rb] = ra
                size[ra] += size[rb]
                count -= 1
        return count
```

- **Time:** `O(V + E * α(V))`, effectively linear.
- **Space:** `O(V)`.
