An island is drawn as an `m x n` grid `heights`, where `heights[r][c]` is the elevation of the cell at row `r` and column `c`. The **Pacific Ocean** touches the island's top and left edges, and the **Atlantic Ocean** touches its bottom and right edges.

When rain falls on a cell, the water can move to any of the four orthogonally adjacent cells whose height is **less than or equal to** the current cell's height. Water on a cell next to an ocean can drain straight into that ocean.

Return the coordinates `[r, c]` of every cell from which rainwater can reach **both** oceans. The cells may be listed in any order.

<!-- examples -->

**Constraints:**

- `m == heights.length`
- `n == heights[r].length`
- `1 <= m, n <= 200`
- `0 <= heights[r][c] <= 10^5`
