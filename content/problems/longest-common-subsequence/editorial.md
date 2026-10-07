## Approach 1: Brute-Force Recursion

**Intuition.** Look at the first character of each remaining suffix. If they are equal, it is
always safe to match them and move on in both strings. If not, at least one of the two
characters is not part of the answer, so try dropping each one and keep the better result.

```python
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        def lcs(i: int, j: int) -> int:
            if i == len(text1) or j == len(text2):
                return 0
            if text1[i] == text2[j]:
                return 1 + lcs(i + 1, j + 1)
            return max(lcs(i + 1, j), lcs(i, j + 1))

        return lcs(0, 0)
```

- **Time:** `O(2^(m + n))` in the worst case (for example, two strings with no common characters),
  because the same `(i, j)` pairs are reached along many different paths.
- **Space:** `O(m + n)` for the recursion.

## Approach 2: Memoized Recursion

**Intuition.** The result only depends on the pair `(i, j)`, and there are just `m * n` such
pairs. Cache them.

```python
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        @lru_cache(maxsize=None)
        def lcs(i: int, j: int) -> int:
            if i == len(text1) or j == len(text2):
                return 0
            if text1[i] == text2[j]:
                return 1 + lcs(i + 1, j + 1)
            return max(lcs(i + 1, j), lcs(i, j + 1))

        return lcs(0, 0)
```

- **Time:** `O(m * n)`.
- **Space:** `O(m * n)` for the cache. The recursion can also go `m + n` levels deep, which
  exceeds Python's default recursion limit for the largest inputs unless it is raised.

## Approach 3: Bottom-Up Table

**Intuition.** Let `dp[i][j]` be the LCS length of the prefixes `text1[:i]` and `text2[:j]`.
Empty prefixes have LCS `0`. Otherwise:

- if `text1[i - 1] == text2[j - 1]`, then `dp[i][j] = dp[i - 1][j - 1] + 1`;
- else `dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])`.

Filling rows top to bottom and columns left to right guarantees all three neighbors are ready.

```python
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        m, n = len(text1), len(text2)
        dp = [[0] * (n + 1) for _ in range(m + 1)]
        for i in range(1, m + 1):
            for j in range(1, n + 1):
                if text1[i - 1] == text2[j - 1]:
                    dp[i][j] = dp[i - 1][j - 1] + 1
                else:
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
        return dp[m][n]
```

- **Time:** `O(m * n)`.
- **Space:** `O(m * n)`.

## Approach 4: Two Rolling Rows (Optimal)

**Intuition.** Row `i` of the table only reads row `i - 1`, so keep just the previous row and
build the current one from it. Iterating over the longer string in the outer loop makes the rows
as short as possible.

**Algorithm.**

1. Make `text2` the shorter string (swap if needed); `prev` is a row of `len(text2) + 1` zeros.
2. For each character `ch` of `text1`, build `cur` starting from `[0]`: for each `j`, append
   `prev[j] + 1` on a match, otherwise the larger of `prev[j + 1]` (above) and `cur[j]` (left).
3. Replace `prev` with `cur`. The answer is the last entry of `prev`.

```python
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        if len(text1) < len(text2):
            text1, text2 = text2, text1
        prev = [0] * (len(text2) + 1)
        for ch in text1:
            cur = [0]
            for j, other in enumerate(text2):
                if ch == other:
                    cur.append(prev[j] + 1)
                else:
                    up, left = prev[j + 1], cur[j]
                    cur.append(up if up > left else left)
            prev = cur
        return prev[-1]
```

- **Time:** `O(m * n)` — at most one million cell updates.
- **Space:** `O(min(m, n))` for the two rows.
