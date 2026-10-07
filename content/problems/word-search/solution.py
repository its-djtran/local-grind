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
