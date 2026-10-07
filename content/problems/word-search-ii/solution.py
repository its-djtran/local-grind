class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        root = {}
        for w in words:
            node = root
            for ch in w:
                node = node.setdefault(ch, {})
            node["$"] = w

        m, n = len(board), len(board[0])
        found = []

        def dfs(i, j, parent):
            ch = board[i][j]
            node = parent[ch]
            word = node.pop("$", None)
            if word is not None:
                found.append(word)
            board[i][j] = "#"
            for x, y in ((i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)):
                if 0 <= x < m and 0 <= y < n and board[x][y] in node:
                    dfs(x, y, node)
            board[i][j] = ch
            if not node:
                del parent[ch]

        for i in range(m):
            for j in range(n):
                if board[i][j] in root:
                    dfs(i, j, root)
        return found
