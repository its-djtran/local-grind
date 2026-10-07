You are given an `m x n` binary matrix `mat`. For every cell, find the distance to the **nearest cell containing `0`**, and return these distances as a matrix of the same shape.

Distance is measured in steps between cells that share an edge (up, down, left or right), so two adjacent cells are at distance `1` and a cell containing `0` is at distance `0` from itself.

<!-- examples -->

**Constraints:**

- `m == mat.length`
- `n == mat[i].length`
- `1 <= m, n <= 10^4`
- `1 <= m * n <= 10^4`
- `mat[i][j]` is either `0` or `1`.
- `mat` contains at least one `0`.
