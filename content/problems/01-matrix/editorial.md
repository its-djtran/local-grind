## Approach 1: Breadth-First Search from Every 1

**Intuition.** For each cell containing `1`, run a BFS outward until it reaches a `0`; the BFS
level at which that happens is the answer for the cell.

```python
class Solution:
    def updateMatrix(self, mat: List[List[int]]) -> List[List[int]]:
        m, n = len(mat), len(mat[0])
        dist = [[0] * n for _ in range(m)]
        for i in range(m):
            for j in range(n):
                if mat[i][j] == 0:
                    continue
                seen, queue = {(i, j)}, deque([(i, j, 0)])
                while queue:
                    r, c, d = queue.popleft()
                    if mat[r][c] == 0:
                        dist[i][j] = d
                        break
                    for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                        if 0 <= nr < m and 0 <= nc < n and (nr, nc) not in seen:
                            seen.add((nr, nc))
                            queue.append((nr, nc, d + 1))
        return dist
```

- **Time:** `O((m * n)^2)` — when zeros are rare, each search can cover most of the matrix.
- **Space:** `O(m * n)`.

## Approach 2: Two-Pass Dynamic Programming

**Intuition.** A shortest path from a cell to its nearest zero can be bent into one that first
moves only vertically and then only horizontally, so it arrives from one of four "quadrants".
A top-left to bottom-right sweep using the up and left neighbors handles paths coming from above
and the left; a reverse sweep using the down and right neighbors fixes the rest.

```python
class Solution:
    def updateMatrix(self, mat: List[List[int]]) -> List[List[int]]:
        m, n = len(mat), len(mat[0])
        INF = m + n
        dist = [[0 if mat[i][j] == 0 else INF for j in range(n)] for i in range(m)]
        for i in range(m):
            for j in range(n):
                if i > 0:
                    dist[i][j] = min(dist[i][j], dist[i - 1][j] + 1)
                if j > 0:
                    dist[i][j] = min(dist[i][j], dist[i][j - 1] + 1)
        for i in range(m - 1, -1, -1):
            for j in range(n - 1, -1, -1):
                if i < m - 1:
                    dist[i][j] = min(dist[i][j], dist[i + 1][j] + 1)
                if j < n - 1:
                    dist[i][j] = min(dist[i][j], dist[i][j + 1] + 1)
        return dist
```

- **Time:** `O(m * n)` — two sweeps.
- **Space:** `O(1)` beyond the output matrix.

## Approach 3: Multi-Source Breadth-First Search (Optimal)

**Intuition.** Flip the question around: instead of each `1` searching for a zero, let all zeros
search outward *simultaneously*. Put every zero in the queue at distance `0`; BFS then reaches
cells in non-decreasing order of distance to the nearest zero, so the first time a cell is
reached, its distance is final.

**Algorithm.**

1. Set `dist` to `0` for zeros and `-1` (unvisited) for ones; enqueue all zeros.
2. Pop a cell; for each unvisited neighbor, set its distance to the current distance plus one and
   enqueue it.
3. Return `dist` once the queue is empty.

```python
class Solution:
    def updateMatrix(self, mat: List[List[int]]) -> List[List[int]]:
        m, n = len(mat), len(mat[0])
        dist = [[-1] * n for _ in range(m)]
        queue = deque()
        for i in range(m):
            for j in range(n):
                if mat[i][j] == 0:
                    dist[i][j] = 0
                    queue.append((i, j))
        while queue:
            r, c = queue.popleft()
            d = dist[r][c] + 1
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < m and 0 <= nc < n and dist[nr][nc] == -1:
                    dist[nr][nc] = d
                    queue.append((nr, nc))
        return dist
```

- **Time:** `O(m * n)` — every cell enters the queue exactly once.
- **Space:** `O(m * n)` for the queue (plus the output).
