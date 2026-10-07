You are given an `m x n` grid of letters `board` and a string `word`. Return `true` if `word` can be spelled by a path through the grid, and `false` otherwise.

A path starts at any cell and steps from one cell to a **horizontally or vertically adjacent** cell each time (no diagonal moves). The letters along the path, in order, must spell `word`. A single cell may appear **at most once** in the path.

<!-- examples -->

**Constraints:**

- `m == board.length`
- `n == board[i].length`
- `1 <= m, n <= 6`
- `1 <= word.length <= 15`
- `board` and `word` contain only uppercase and lowercase English letters.

**Follow-up:** How would you prune the search so that it stays fast on a much larger board?
