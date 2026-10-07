You are given an `m x n` grid of letters, `board`, and a list of distinct strings, `words`. Return every word from the list that can be found on the board.

A word is found when it can be spelled by moving through a sequence of cells, starting anywhere, where each step goes to a cell that is **horizontally or vertically adjacent** to the previous one. A single cell may be used **at most once** within the same word.

The words may be returned in any order.

<!-- examples -->

**Constraints:**

- `m == board.length`
- `n == board[i].length`
- `1 <= m, n <= 12`
- `board[i][j]` is a lowercase English letter.
- `1 <= words.length <= 3 * 10^4`
- `1 <= words[i].length <= 10`
- `words[i]` consists of lowercase English letters.
- All the strings in `words` are unique.
