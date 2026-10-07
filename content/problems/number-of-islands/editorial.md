Every approach scans the grid and, at each unvisited land cell, counts a new island and then
claims the whole island so it is not counted again. They differ in how the island is claimed.

## Approach 1: Recursive Depth-First Search

**Intuition.** From a land cell, recursively visit its four neighbors that are land, overwriting
each visited cell with `'0'` ("sinking" it). When the recursion returns, the island is gone.

```python
class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])

        def sink(r, c):
            if r < 0 or r >= m or c < 0 or c >= n or grid[r][c] != "1":
                return
            grid[r][c] = "0"
            sink(r + 1, c)
            sink(r - 1, c)
            sink(r, c + 1)
            sink(r, c - 1)

        count = 0
        for r in range(m):
            for c in range(n):
                if grid[r][c] == "1":
                    count += 1
                    sink(r, c)
        return count
```

- **Time:** `O(m * n)` — every cell is sunk at most once and examined a constant number of times.
- **Space:** `O(m * n)` recursion depth in the worst case (a snake-shaped island covering the
  grid). In many languages that overflows the call stack, which motivates the next approach.

## Approach 2: Union-Find

**Intuition.** Treat every land cell as its own component, then union each land cell with its
land neighbors to the right and below. Each successful union merges two components, so the
answer is the number of land cells minus the number of successful unions.

```python
class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])
        parent = list(range(m * n))

        def find(x):
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        count = 0
        for r in range(m):
            for c in range(n):
                if grid[r][c] != "1":
                    continue
                count += 1
                for nr, nc in ((r + 1, c), (r, c + 1)):
                    if nr < m and nc < n and grid[nr][nc] == "1":
                        a, b = find(r * n + c), find(nr * n + nc)
                        if a != b:
                            parent[a] = b
                            count -= 1
        return count
```

- **Time:** `O(m * n * α(m * n))`, effectively linear.
- **Space:** `O(m * n)` for the parent array.

## Approach 3: Iterative Flood Fill (Optimal)

**Intuition.** Same as Approach 1, but with an explicit stack so the depth of the island no longer
matters. Mark a cell as water when it is *pushed*, so no cell is pushed twice.

```python
class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])
        count = 0
        for i in range(m):
            for j in range(n):
                if grid[i][j] != "1":
                    continue
                count += 1
                grid[i][j] = "0"
                stack = [(i, j)]
                while stack:
                    r, c = stack.pop()
                    for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                        if 0 <= nr < m and 0 <= nc < n and grid[nr][nc] == "1":
                            grid[nr][nc] = "0"
                            stack.append((nr, nc))
        return count
```

- **Time:** `O(m * n)`.
- **Space:** `O(m * n)` for the stack in the worst case (`O(1)` extra besides it, since visited
  marks are stored in the grid). If modifying the input is not allowed, use a separate `visited`
  grid instead.
