## Approach 1: Search for Each Word Separately (Brute Force)

**Intuition.** Solve the single-word problem (Word Search) once per word: try every cell as a
starting point and backtrack through neighbours that match the next letter.

```python
class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        m, n = len(board), len(board[0])

        def exist(word):
            def dfs(i, j, k):
                if k == len(word):
                    return True
                if not (0 <= i < m and 0 <= j < n) or board[i][j] != word[k]:
                    return False
                board[i][j] = "#"
                ok = (dfs(i + 1, j, k + 1) or dfs(i - 1, j, k + 1)
                      or dfs(i, j + 1, k + 1) or dfs(i, j - 1, k + 1))
                board[i][j] = word[k]
                return ok

            return any(dfs(i, j, 0) for i in range(m) for j in range(n))

        return [w for w in words if exist(w)]
```

- **Time:** `O(W * m * n * 3^L)` for `W` words of length up to `L`: every word restarts from
  every cell. Even when most searches die immediately, `30000 * 144` start attempts alone are
  millions of calls.
- **Space:** `O(L)` for the recursion.

## Approach 2: Backtracking over a Trie (Optimal)

**Intuition.** Many words share prefixes, and every word restarts the same exploration in
Approach 1. Instead, put all words in a trie and run a single family of depth-first searches over
the board. While walking the board we simultaneously walk the trie: a path continues only while
its letters spell a prefix of at least one word, and whenever we reach a trie node that ends a
word, that word is found.

**Algorithm.**

1. Build a trie of the words; store the complete word at the node where it ends.
2. From every cell whose letter is a child of the root, run `dfs(i, j, parent)`:
   - Move to the trie child for `board[i][j]`. If it stores a word, record it and remove it from
     the node so it is reported only once.
   - Temporarily replace the cell with `#` so the path cannot reuse it, recurse into neighbours
     whose letters are children of the current trie node, then restore the cell.
   - **Prune:** if the trie node has become empty (no children, no word left), delete it from its
     parent. Paths that can no longer yield new words are never explored again.

Pruning matters on repetitive boards. On a board full of `a` with the words `a`, `aa`, ...,
`aaaaaaaaaa`, the first deep path finds all ten words and then removes the whole branch; without
pruning, the search would keep enumerating hundreds of thousands of paths for nothing.

```python
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
```

- **Time:** `O(total letters in words)` to build the trie, plus `O(m * n * 4 * 3^(L-1))` for the
  board search in the worst case, where `L <= 10`. In practice the trie limits exploration to
  paths that are prefixes of real words, and pruning removes finished branches.
- **Space:** `O(total letters in words)` for the trie, plus `O(L)` recursion depth.
