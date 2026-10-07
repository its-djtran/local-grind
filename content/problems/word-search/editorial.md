## Approach 1: Backtracking Depth-First Search

**Intuition.** Try every cell as the starting point. From a cell that matches `word[k]`, the
rest of the word `word[k+1:]` must be found starting at one of its four neighbours, without
reusing any cell already on the path. That is a natural recursive search: mark the cell as used,
explore the neighbours, then unmark it on the way back (backtrack) so that other paths may use
it.

**Algorithm.**

1. Define `dfs(i, j, k)`: can `word[k:]` be spelled starting at cell `(i, j)`?
   - If `(i, j)` is outside the grid or `board[i][j] != word[k]`, return `False`.
   - If `k` is the last index, return `True`.
   - Temporarily overwrite `board[i][j]` with a marker such as `"#"` so it cannot match again,
     recurse into the four neighbours with `k + 1`, restore the letter, and return whether any
     neighbour succeeded.
2. Return `True` if `dfs(i, j, 0)` succeeds for any cell.

```python
class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m, n = len(board), len(board[0])

        def dfs(i, j, k):
            if not (0 <= i < m and 0 <= j < n) or board[i][j] != word[k]:
                return False
            if k == len(word) - 1:
                return True
            board[i][j] = "#"
            found = (dfs(i + 1, j, k + 1) or dfs(i - 1, j, k + 1)
                     or dfs(i, j + 1, k + 1) or dfs(i, j - 1, k + 1))
            board[i][j] = word[k]
            return found

        return any(dfs(i, j, 0) for i in range(m) for j in range(n))
```

- **Time:** `O(m * n * 3^L)` where `L = len(word)` — after the first step each cell has at most
  three unvisited neighbours to try. On adversarial inputs, such as a board full of `A` and the
  word `AAAAAAAAAAAAAAB`, the search really does explore millions of paths before failing.
- **Space:** `O(L)` for the recursion stack.

## Approach 2: Backtracking With Pruning (Optimal)

**Intuition.** The worst cases for plain backtracking are words that *almost* match
everywhere. Two cheap checks remove most of them before any search happens:

- **Letter counts.** If the word needs more copies of some letter than the board contains, no
  path can exist. This also covers `len(word) > m * n`.
- **Search from the rarer end.** A path spelling `word` read backwards is the same path, so we
  may search for the reversed word instead. Starting from the end whose letter is less common
  on the board means fewer starting cells, and mismatches tend to appear near the root of the
  search tree instead of deep inside it.

The search itself is the same backtracking DFS.

**Algorithm.**

1. Return `False` if the word is longer than the board or any letter count is insufficient.
2. If `word[0]` is more frequent on the board than `word[-1]`, reverse `word`.
3. Run the backtracking DFS from every cell.

```python
class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m, n = len(board), len(board[0])
        if len(word) > m * n:
            return False
        have = Counter(ch for row in board for ch in row)
        need = Counter(word)
        if any(have[ch] < c for ch, c in need.items()):
            return False
        # Start from the end whose letter is rarer on the board: fewer roots, earlier failures.
        if have[word[0]] > have[word[-1]]:
            word = word[::-1]
        last = len(word) - 1

        def dfs(i, j, k):
            if board[i][j] != word[k]:
                return False
            if k == last:
                return True
            board[i][j] = "#"
            found = (
                (i > 0 and dfs(i - 1, j, k + 1))
                or (i + 1 < m and dfs(i + 1, j, k + 1))
                or (j > 0 and dfs(i, j - 1, k + 1))
                or (j + 1 < n and dfs(i, j + 1, k + 1))
            )
            board[i][j] = word[k]
            return found

        return any(dfs(i, j, 0) for i in range(m) for j in range(n))
```

- **Time:** still `O(m * n * 3^L)` in the worst case, but the pruning removes the classic
  adversarial inputs, and the counting step costs only `O(mn + L)`.
- **Space:** `O(L)` for the recursion plus `O(1)` for the letter counts (52 letters).
