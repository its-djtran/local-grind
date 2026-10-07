## Approach 1: Brute-Force Recursion

**Intuition.** The last move into the target cell came either from the cell directly above or the
cell directly to the left, so the number of routes to a cell is the sum of the routes to those two
neighbors. A cell in the first row or first column can be reached in only one way.

```python
class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        def paths(r: int, c: int) -> int:
            if r == 0 or c == 0:
                return 1
            return paths(r - 1, c) + paths(r, c - 1)

        return paths(m - 1, n - 1)
```

- **Time:** proportional to the answer itself (every route ends in its own base-case call), which
  can be about two billion.
- **Space:** `O(m + n)` for the recursion.

## Approach 2: 2-D Dynamic Programming

**Intuition.** The recursion asks about the same cells over and over. Fill a table of route counts
row by row instead, so each cell is computed once from its already-known top and left neighbors.

```python
class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        dp = [[1] * n for _ in range(m)]
        for r in range(1, m):
            for c in range(1, n):
                dp[r][c] = dp[r - 1][c] + dp[r][c - 1]
        return dp[m - 1][n - 1]
```

- **Time:** `O(m * n)`.
- **Space:** `O(m * n)`.

## Approach 3: One Rolling Row

**Intuition.** Row `r` only needs row `r - 1`. In a single array `row`, before the update `row[c]`
still holds the value from the row above, and `row[c - 1]` already holds the new value to the left,
so `row[c] += row[c - 1]` is exactly the recurrence.

```python
class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        row = [1] * n
        for _ in range(1, m):
            for c in range(1, n):
                row[c] += row[c - 1]
        return row[-1]
```

- **Time:** `O(m * n)`.
- **Space:** `O(n)`.

## Approach 4: Counting Arrangements (Optimal)

**Intuition.** Every route consists of exactly `m - 1` down moves and `n - 1` right moves, and any
ordering of those moves is a valid route that stays inside the grid. So the answer is the number of
ways to choose which `m - 1` of the `m + n - 2` moves are "down": the binomial coefficient
`C(m + n - 2, m - 1)`.

**Algorithm.** Compute the binomial coefficient with the smaller of the two counts, multiplying and
dividing step by step. After step `i` the running value is `C(total - k + i, i)`, an integer, so the
integer division is always exact.

```python
class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        total, k = m + n - 2, min(m, n) - 1
        result = 1
        for i in range(1, k + 1):
            result = result * (total - k + i) // i
        return result
```

- **Time:** `O(min(m, n))`.
- **Space:** `O(1)`.

(`math.comb(m + n - 2, m - 1)` computes the same value in one call.)
