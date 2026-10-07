## Approach 1: Search from Every Cell

**Intuition.** Follow the rules literally: for each cell, explore every cell its water can reach
(moving to neighbors that are not higher) and note whether that exploration ever touches the top
or left edge (Pacific) and the bottom or right edge (Atlantic).

```python
class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        m, n = len(heights), len(heights[0])
        result = []
        for sr in range(m):
            for sc in range(n):
                pacific = atlantic = False
                seen, stack = {(sr, sc)}, [(sr, sc)]
                while stack and not (pacific and atlantic):
                    r, c = stack.pop()
                    pacific |= r == 0 or c == 0
                    atlantic |= r == m - 1 or c == n - 1
                    for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                        if (0 <= nr < m and 0 <= nc < n and (nr, nc) not in seen
                                and heights[nr][nc] <= heights[r][c]):
                            seen.add((nr, nc))
                            stack.append((nr, nc))
                if pacific and atlantic:
                    result.append([sr, sc])
        return result
```

- **Time:** `O((m * n)^2)` — each of the `m * n` searches may visit the whole grid. On a
  `200 x 200` grid that is over a billion steps.
- **Space:** `O(m * n)`.

## Approach 2: Reverse Search from the Oceans (Optimal)

**Intuition.** Instead of asking "where can this cell's water go?" for every cell, ask "which
cells can send water *to* this ocean?". Water flows from a cell to a neighbor that is not higher,
so walking backwards we move from a cell to a neighbor that is **at least as high**. Starting from
every cell on the Pacific edges and climbing gives exactly the set of cells that drain into the
Pacific; doing the same from the Atlantic edges gives the other set. The answer is their
intersection.

**Algorithm.**

1. Seed a BFS with all cells in row `0` and column `0` (Pacific), and another with all cells in
   row `m - 1` and column `n - 1` (Atlantic).
2. From each dequeued cell, enqueue unvisited neighbors whose height is `>=` the current height.
3. Return every cell visited by both searches.

```python
class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        m, n = len(heights), len(heights[0])

        def climb(starts):
            seen = [[False] * n for _ in range(m)]
            queue = deque()
            for r, c in starts:
                if not seen[r][c]:
                    seen[r][c] = True
                    queue.append((r, c))
            while queue:
                r, c = queue.popleft()
                h = heights[r][c]
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < m and 0 <= nc < n and not seen[nr][nc] and heights[nr][nc] >= h:
                        seen[nr][nc] = True
                        queue.append((nr, nc))
            return seen

        pacific = climb([(0, c) for c in range(n)] + [(r, 0) for r in range(m)])
        atlantic = climb([(m - 1, c) for c in range(n)] + [(r, n - 1) for r in range(m)])
        return [[r, c] for r in range(m) for c in range(n) if pacific[r][c] and atlantic[r][c]]
```

- **Time:** `O(m * n)` — each search visits every cell at most once.
- **Space:** `O(m * n)` for the two visited grids and the queue.
