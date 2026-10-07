After a clockwise quarter turn, the value at row `i`, column `j` moves to row `j`, column
`n - 1 - i`. Equivalently, the new row `i` is the old column `i` read from bottom to top.

## Approach 1: Rotate Four Cells at a Time

**Intuition.** Following the mapping above, every cell belongs to a cycle of four positions
that trade places around a ring: top-left corner region to top-right, top-right to
bottom-right, bottom-right to bottom-left, bottom-left back to top-left. Saving one value in a
temporary lets us shift the other three along the cycle without losing anything.

**Algorithm.**

1. Process the rings from the outside in: ring `i` covers rows and columns `i` to `n - 1 - i`.
2. For each offset `j` from `i` to `n - 2 - i`, rotate the four cells
   `(i, j)`, `(j, n-1-i)`, `(n-1-i, n-1-j)`, `(n-1-j, i)` one step clockwise.

```python
class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        n = len(matrix)
        for i in range(n // 2):
            for j in range(i, n - 1 - i):
                tmp = matrix[i][j]
                matrix[i][j] = matrix[n - 1 - j][i]
                matrix[n - 1 - j][i] = matrix[n - 1 - i][n - 1 - j]
                matrix[n - 1 - i][n - 1 - j] = matrix[j][n - 1 - i]
                matrix[j][n - 1 - i] = tmp
```

- **Time:** `O(n^2)` — each cell is moved once.
- **Space:** `O(1)`.

## Approach 2: Transpose, Then Reverse Each Row (Optimal)

**Intuition.** Transposing sends `(i, j)` to `(j, i)`. Reversing a row then sends column `i` to
column `n - 1 - i`. Combined, `(i, j)` lands at `(j, n - 1 - i)`, which is exactly the clockwise
rotation. Both steps are easy to do in place and hard to get wrong.

**Algorithm.**

1. For every pair `i < j`, swap `matrix[i][j]` with `matrix[j][i]`.
2. Reverse every row.

```python
class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        n = len(matrix)
        for i in range(n):
            for j in range(i + 1, n):
                matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
        for row in matrix:
            row.reverse()
```

- **Time:** `O(n^2)`.
- **Space:** `O(1)`.

For a counter-clockwise rotation, reverse each row first and then transpose (or transpose and
reverse the order of the rows).
