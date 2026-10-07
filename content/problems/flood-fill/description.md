An image is stored as an `m x n` grid of integers, where `image[i][j]` is the color of the pixel in row `i` and column `j`. You are also given a starting pixel `(sr, sc)` and a new color `color`.

Perform a **flood fill** starting at `(sr, sc)`: let `original` be the starting pixel's color. Repaint the starting pixel with `color`, and keep spreading to any pixel that is directly above, below, left or right of an already-repainted pixel and whose color is `original`. Diagonal neighbors are not included.

Return the image after the fill.

<!-- examples -->

**Constraints:**

- `m == image.length`
- `n == image[i].length`
- `1 <= m, n <= 50`
- `0 <= image[i][j], color < 2^16`
- `0 <= sr < m`
- `0 <= sc < n`
