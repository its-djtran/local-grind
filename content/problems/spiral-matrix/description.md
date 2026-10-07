Given an `m x n` grid `matrix`, return all of its values in **spiral order**.

Spiral order starts at the top-left cell and moves right along the top row, then down the right-hand column, then left along the bottom row, then up the left-hand column. It then continues the same clockwise pattern on the part of the grid that has not been visited yet, until every cell has been listed exactly once.

<!-- examples -->

**Constraints:**

- `m == matrix.length`
- `n == matrix[i].length`
- `1 <= m, n <= 10`
- `-100 <= matrix[i][j] <= 100`
