Draw an arrow `b -> a` for every pair `[a, b]`. All courses can be finished exactly when this
directed graph has **no cycle**: a cycle is a set of courses that each wait on another member of
the set, while an acyclic graph always has some course with nothing left to wait for.

## Approach 1: Cycle Search from Every Course

**Intuition.** A course lies on a cycle if, starting from it and following arrows, we can get back
to it. Check that for every course.

**Algorithm.** For each course `s`, run an iterative DFS over the arrows starting at `s`'s
successors with a fresh `visited` set. If `s` is ever reached, return `false`. If no start finds
itself, return `true`.

```python
class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        graph = [[] for _ in range(numCourses)]
        for a, b in prerequisites:
            graph[b].append(a)
        for s in range(numCourses):
            seen, stack = set(), list(graph[s])
            while stack:
                v = stack.pop()
                if v == s:
                    return False
                if v not in seen:
                    seen.add(v)
                    stack.extend(graph[v])
        return True
```

- **Time:** `O(V * (V + E))` — a full traversal per course. With `V = 2000` and `E = 5000`
  this is far too slow.
- **Space:** `O(V + E)`.

## Approach 2: DFS with Three Colors

**Intuition.** One traversal is enough if we remember what each node's DFS already proved.
Color nodes white (unvisited), gray (on the current DFS path), or black (fully explored, and no
cycle is reachable from it). Reaching a gray node means we have walked around a cycle; reaching a
black node needs no further work.

**Algorithm.** For every white node start a DFS. Mark it gray, visit all successors, return
`false` on a gray successor, then mark it black.

```python
class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        graph = [[] for _ in range(numCourses)]
        for a, b in prerequisites:
            graph[b].append(a)
        WHITE, GRAY, BLACK = 0, 1, 2
        color = [WHITE] * numCourses

        def has_cycle(v):
            color[v] = GRAY
            for w in graph[v]:
                if color[w] == GRAY or (color[w] == WHITE and has_cycle(w)):
                    return True
            color[v] = BLACK
            return False

        return not any(color[v] == WHITE and has_cycle(v) for v in range(numCourses))
```

- **Time:** `O(V + E)`.
- **Space:** `O(V + E)` including the recursion stack.

## Approach 3: Kahn's Algorithm (Optimal, Iterative)

**Intuition.** Simulate taking courses. A course with in-degree `0` has no unmet prerequisites,
so take it and "remove" its outgoing arrows, which may free other courses. If a cycle exists, its
courses never reach in-degree `0`.

**Algorithm.**

1. Build the adjacency list and the in-degree of every course.
2. Push every course with in-degree `0` onto a queue.
3. Pop a course, count it as taken, and decrement each successor's in-degree, enqueuing any that
   reach `0`.
4. Return whether the number of taken courses equals `numCourses`.

```python
class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        graph = [[] for _ in range(numCourses)]
        indegree = [0] * numCourses
        for a, b in prerequisites:
            graph[b].append(a)
            indegree[a] += 1
        queue = deque(v for v in range(numCourses) if indegree[v] == 0)
        taken = 0
        while queue:
            v = queue.popleft()
            taken += 1
            for w in graph[v]:
                indegree[w] -= 1
                if indegree[w] == 0:
                    queue.append(w)
        return taken == numCourses
```

- **Time:** `O(V + E)` — every course and every prerequisite is processed once.
- **Space:** `O(V + E)`.
