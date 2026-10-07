You are given an `m x n` grid where each cell holds one of three values:

- `0`: an empty cell,
- `1`: a fresh orange,
- `2`: a rotten orange.

Each minute, every fresh orange that is directly above, below, left or right of a rotten orange becomes rotten.

Return the minimum number of minutes that must pass until no fresh orange remains. If some fresh orange can never rot, return `-1`.

<!-- examples -->

**Constraints:**

- `m == grid.length`
- `n == grid[i].length`
- `1 <= m, n <= 10`
- `grid[i][j]` is `0`, `1`, or `2`.
