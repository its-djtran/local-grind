## Approach 1: Simulate With a Visited Grid

**Intuition.** Walk the grid the way a person would trace the spiral with a finger: keep
moving in the current direction, and turn clockwise whenever the next cell is outside the grid
or has already been visited.

**Algorithm.**

1. Keep the four directions in clockwise order: right, down, left, up.
2. Starting at `(0, 0)` facing right, record the current cell and mark it visited.
3. If the cell ahead is out of bounds or visited, rotate to the next direction. Step forward.
4. Stop after `m * n` cells have been recorded.

```python
class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        m, n = len(matrix), len(matrix[0])
        seen = [[False] * n for _ in range(m)]
        dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        i = j = d = 0
        out = []
        for _ in range(m * n):
            out.append(matrix[i][j])
            seen[i][j] = True
            ni, nj = i + dirs[d][0], j + dirs[d][1]
            if not (0 <= ni < m and 0 <= nj < n) or seen[ni][nj]:
                d = (d + 1) % 4
                ni, nj = i + dirs[d][0], j + dirs[d][1]
            i, j = ni, nj
        return out
```

- **Time:** `O(mn)` — each cell is visited once.
- **Space:** `O(mn)` for the visited grid (the output list is not counted).

## Approach 2: Shrinking Boundaries (Optimal)

**Intuition.** The spiral is a sequence of rectangular rings. If we track the rectangle that is
still unvisited through four indices `top`, `bottom`, `left`, `right`, each ring is just four
straight walks, and afterwards every boundary moves one step inward. No visited grid is
needed.

**Algorithm.**

1. While `top <= bottom` and `left <= right`:
   - Walk the top row from `left` to `right`.
   - Walk the right column from `top + 1` to `bottom`.
   - If more than one row remains (`top < bottom`), walk the bottom row from `right - 1` back
     to `left`.
   - If more than one column remains (`left < right`), walk the left column from `bottom - 1`
     up to `top + 1`.
   - Shrink all four boundaries.

The two guards matter for the last ring when it is a single row or a single column; without
them those cells would be emitted twice.

```python
class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        top, bottom = 0, len(matrix) - 1
        left, right = 0, len(matrix[0]) - 1
        out = []
        while top <= bottom and left <= right:
            for j in range(left, right + 1):
                out.append(matrix[top][j])
            for i in range(top + 1, bottom + 1):
                out.append(matrix[i][right])
            if top < bottom:
                for j in range(right - 1, left - 1, -1):
                    out.append(matrix[bottom][j])
            if left < right:
                for i in range(bottom - 1, top, -1):
                    out.append(matrix[i][left])
            top, bottom, left, right = top + 1, bottom - 1, left + 1, right - 1
        return out
```

- **Time:** `O(mn)`.
- **Space:** `O(1)` besides the output.
