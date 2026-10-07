## Approach 1: Clear From a Snapshot

**Intuition.** The danger is confusing zeros that were in the input with zeros we have just
written. Taking a full copy of the grid first removes that confusion: scan the copy for zeros
and clear rows and columns in the real grid.

**Algorithm.**

1. Copy every row of `matrix` into `original`.
2. For every cell where `original[i][j] == 0`, set all of row `i` and all of column `j` in
   `matrix` to `0`.

```python
class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        m, n = len(matrix), len(matrix[0])
        original = [row[:] for row in matrix]
        for i in range(m):
            for j in range(n):
                if original[i][j] == 0:
                    for c in range(n):
                        matrix[i][c] = 0
                    for r in range(m):
                        matrix[r][j] = 0
```

- **Time:** `O(mn(m + n))` — each original zero clears a full row and column, and there can be
  `O(mn)` zeros.
- **Space:** `O(mn)` for the copy.

## Approach 2: Row and Column Flags

**Intuition.** A cell ends up `0` exactly when its row or its column contained a zero. So we
only need two facts per cell: "does row `i` have a zero?" and "does column `j` have a zero?".

**Algorithm.**

1. Scan the grid once; for every zero at `(i, j)` mark `rows[i]` and `cols[j]`.
2. Scan again and set `matrix[i][j] = 0` whenever `rows[i]` or `cols[j]` is marked.

```python
class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        m, n = len(matrix), len(matrix[0])
        rows, cols = [False] * m, [False] * n
        for i in range(m):
            for j in range(n):
                if matrix[i][j] == 0:
                    rows[i] = cols[j] = True
        for i in range(m):
            for j in range(n):
                if rows[i] or cols[j]:
                    matrix[i][j] = 0
```

- **Time:** `O(mn)` — two passes over the grid.
- **Space:** `O(m + n)` for the flags.

## Approach 3: Flags Stored in the First Row and Column (Optimal)

**Intuition.** The first row has one cell per column and the first column has one cell per
row, so they can hold the flags from Approach 2. The only catch is that they are also data: we
must separately remember whether the first row and first column themselves need clearing,
before we overwrite them with flags.

**Algorithm.**

1. Record `first_row` = "row 0 contains a zero" and `first_col` = "column 0 contains a zero".
2. For every cell `(i, j)` with `i, j >= 1` that is zero, write `matrix[i][0] = 0` and
   `matrix[0][j] = 0`.
3. For every cell `(i, j)` with `i, j >= 1`, set it to `0` if `matrix[i][0] == 0` or
   `matrix[0][j] == 0`.
4. Finally clear row 0 if `first_row` is set and column 0 if `first_col` is set. This happens
   last so the flags are not destroyed while they are still needed.

```python
class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        m, n = len(matrix), len(matrix[0])
        first_row = any(v == 0 for v in matrix[0])
        first_col = any(matrix[i][0] == 0 for i in range(m))
        for i in range(1, m):
            row = matrix[i]
            for j in range(1, n):
                if row[j] == 0:
                    row[0] = 0
                    matrix[0][j] = 0
        top = matrix[0]
        for i in range(1, m):
            row = matrix[i]
            if row[0] == 0:
                for j in range(1, n):
                    row[j] = 0
            else:
                for j in range(1, n):
                    if top[j] == 0:
                        row[j] = 0
        if first_row:
            for j in range(n):
                top[j] = 0
        if first_col:
            for i in range(m):
                matrix[i][0] = 0
```

- **Time:** `O(mn)`.
- **Space:** `O(1)` beyond the grid itself.
