## Approach 1: Minute-by-Minute Simulation

**Intuition.** Literally replay the process. Each minute, scan the grid and collect every fresh
orange next to a rotten one, then rot them all at once (collecting first, so oranges rotted this
minute do not spread until the next). Stop when a minute changes nothing.

```python
class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        minutes = 0
        while True:
            to_rot = [
                (r, c) for r in range(m) for c in range(n)
                if grid[r][c] == 1 and any(
                    0 <= nr < m and 0 <= nc < n and grid[nr][nc] == 2
                    for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)))
            ]
            if not to_rot:
                break
            for r, c in to_rot:
                grid[r][c] = 2
            minutes += 1
        return -1 if any(1 in row for row in grid) else minutes
```

- **Time:** `O((m * n)^2)` — up to `m * n` minutes, each scanning the whole grid.
- **Space:** `O(m * n)` for the list of oranges rotted in one minute.

## Approach 2: Multi-Source Breadth-First Search (Optimal)

**Intuition.** The simulation rescans cells that cannot change. BFS visits each orange once: start
with *all* rotten oranges in the queue (they spread simultaneously), and process the queue one
level at a time. Level `k` contains exactly the oranges that rot at minute `k`.

**Algorithm.**

1. Count fresh oranges and enqueue every rotten one.
2. While the queue is non-empty and fresh oranges remain: process the whole current level,
   rotting each fresh neighbor (decrementing the fresh count and enqueuing it), then add one
   minute.
3. Return the minutes if no fresh orange remains, otherwise `-1`.

Stopping as soon as `fresh` reaches zero avoids counting an extra minute for the final level,
which rots nothing new.

```python
class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        queue = deque()
        fresh = 0
        for r in range(m):
            for c in range(n):
                if grid[r][c] == 2:
                    queue.append((r, c))
                elif grid[r][c] == 1:
                    fresh += 1
        minutes = 0
        while queue and fresh:
            for _ in range(len(queue)):
                r, c = queue.popleft()
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < m and 0 <= nc < n and grid[nr][nc] == 1:
                        grid[nr][nc] = 2
                        fresh -= 1
                        queue.append((nr, nc))
            minutes += 1
        return minutes if fresh == 0 else -1
```

- **Time:** `O(m * n)` — each cell is enqueued at most once.
- **Space:** `O(m * n)` for the queue.
