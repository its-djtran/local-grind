## Approach 1: Brute Force

**Intuition.** Test every substring and keep the longest one that equals its reverse.

```python
class Solution:
    def longestPalindrome(self, s: str) -> str:
        best = ""
        for i in range(len(s)):
            for j in range(i + len(best) + 1, len(s) + 1):
                sub = s[i:j]
                if sub == sub[::-1]:
                    best = sub
        return best
```

- **Time:** `O(n^3)` — about `n^2 / 2` substrings, each checked in `O(n)`.
- **Space:** `O(n)` for the substring being checked.

## Approach 2: Dynamic Programming

**Intuition.** `s[i..j]` is a palindrome exactly when its end characters match and the inside
`s[i+1..j-1]` is a palindrome (or is shorter than two characters). Filling a table by increasing
start index from the right lets each cell reuse an already computed answer.

**Algorithm.** Let `pal[i][j]` be true when `s[i..j]` is a palindrome. Iterate `i` from `n - 1`
down to `0` and `j` from `i` up to `n - 1`, setting
`pal[i][j] = s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1])`, and track the longest true cell.

```python
class Solution:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        pal = [[False] * n for _ in range(n)]
        start, length = 0, 1
        for i in range(n - 1, -1, -1):
            for j in range(i, n):
                if s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1]):
                    pal[i][j] = True
                    if j - i + 1 > length:
                        start, length = i, j - i + 1
        return s[start:start + length]
```

- **Time:** `O(n^2)`.
- **Space:** `O(n^2)` for the table.

## Approach 3: Expand Around Centers

**Intuition.** Every palindrome is symmetric about a center, which is either one character (odd
length) or the gap between two neighbours (even length). Growing outward from each of the
`2n - 1` centers while both sides agree finds the longest palindrome at that center, with no
table at all.

**Algorithm.**

1. For each index `i`, expand from `(i, i)` and from `(i, i + 1)`: while both indices are in range
   and the characters match, move left down and right up.
2. The palindrome found is `s[left + 1 : right]`; keep the longest.

```python
class Solution:
    def longestPalindrome(self, s: str) -> str:
        def expand(lo: int, hi: int) -> tuple:
            while lo >= 0 and hi < len(s) and s[lo] == s[hi]:
                lo -= 1
                hi += 1
            return lo + 1, hi

        best = (0, 1)
        for i in range(len(s)):
            for lo, hi in (expand(i, i), expand(i, i + 1)):
                if hi - lo > best[1] - best[0]:
                    best = (lo, hi)
        return s[best[0]:best[1]]
```

- **Time:** `O(n^2)` in the worst case (a string of one repeated letter), and close to `O(n)` on
  typical strings where palindromes are short.
- **Space:** `O(1)` besides the output.

## Approach 4: Manacher's Algorithm (Optimal)

**Intuition.** Insert a separator between all characters (`"aba"` becomes `"^#a#b#a#$"`), so every
palindrome has odd length and a single center. Let `radius[i]` be how far the palindrome centered
at `i` extends. If `i` lies inside a palindrome already found, centered at `center` and reaching
`right`, then its mirror `2 * center - i` was computed earlier, and the palindrome at `i` is at
least as long as the mirror's, clipped to `right - i`. Only characters past `right` need to be
compared, and `right` never moves backwards, so the total work is linear.

**Algorithm.**

1. Build `t = "^#" + "#".join(s) + "#$"`. The distinct sentinels `^` and `$` stop expansion
   without bounds checks.
2. For each `i` from `1` to `len(t) - 2`: start `radius[i]` at `min(right - i, radius[mirror])`
   when `i < right` (else `0`), then expand while `t[i + radius + 1] == t[i - radius - 1]`. If the
   palindrome now reaches past `right`, update `center` and `right`.
3. The largest `radius[i]` is the length of the longest palindrome in `s`, and it starts at index
   `(i - radius[i]) // 2` in `s`.

```python
class Solution:
    def longestPalindrome(self, s: str) -> str:
        t = "^#" + "#".join(s) + "#$"
        radius = [0] * len(t)
        center = right = 0
        for i in range(1, len(t) - 1):
            if i < right:
                radius[i] = min(right - i, radius[2 * center - i])
            while t[i + radius[i] + 1] == t[i - radius[i] - 1]:
                radius[i] += 1
            if i + radius[i] > right:
                center, right = i, i + radius[i]
        length, mid = max((r, i) for i, r in enumerate(radius))
        start = (mid - length) // 2
        return s[start:start + length]
```

- **Time:** `O(n)` — every successful comparison pushes `right` forward.
- **Space:** `O(n)` for the transformed string and radii.
