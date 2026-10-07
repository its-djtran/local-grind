## Approach 1: Brute Force

**Intuition.** Enumerate every substring and test whether it equals its reverse.

```python
class Solution:
    def countSubstrings(self, s: str) -> int:
        count = 0
        for i in range(len(s)):
            for j in range(i + 1, len(s) + 1):
                sub = s[i:j]
                if sub == sub[::-1]:
                    count += 1
        return count
```

- **Time:** `O(n^3)` — `n(n + 1) / 2` substrings, each checked in `O(n)`.
- **Space:** `O(n)` for the substring copy.

## Approach 2: Dynamic Programming

**Intuition.** `s[i..j]` is a palindrome when `s[i] == s[j]` and the inner part `s[i+1..j-1]` is a
palindrome (an inner part of length 0 or 1 always is). Record the answer for each pair `(i, j)`
so the inner check is a table lookup instead of a scan.

**Algorithm.** Process `i` from right to left so that row `i + 1` is complete before row `i` needs
it. For each `j >= i`, set `pal[j]` for the current row and add one to the count whenever it is
true. A single row of the table suffices if `j` runs from right to left, because `pal[j - 1]` still
holds the value from row `i + 1` when it is read.

```python
class Solution:
    def countSubstrings(self, s: str) -> int:
        n = len(s)
        pal = [False] * n
        count = 0
        for i in range(n - 1, -1, -1):
            for j in range(n - 1, i - 1, -1):
                pal[j] = s[i] == s[j] and (j - i < 2 or pal[j - 1])
                count += pal[j]
        return count
```

- **Time:** `O(n^2)`.
- **Space:** `O(n)` with the rolling row (`O(n^2)` for the full table).

## Approach 3: Expand Around Centers

**Intuition.** Every palindrome has a center: a character (odd length) or a gap (even length).
Starting at a center and stepping outward one character on each side, each step where the two
new characters match reveals exactly one new palindrome, and the first mismatch ends that center
for good.

```python
class Solution:
    def countSubstrings(self, s: str) -> int:
        n, count = len(s), 0
        for center in range(2 * n - 1):
            lo, hi = center // 2, center // 2 + center % 2
            while lo >= 0 and hi < n and s[lo] == s[hi]:
                count += 1
                lo -= 1
                hi += 1
        return count
```

- **Time:** `O(n^2)` in the worst case (`"aaa...a"`), but each step corresponds to a palindrome
  that is counted, so it is fast whenever the answer is small.
- **Space:** `O(1)`.

## Approach 4: Manacher's Algorithm (Optimal)

**Intuition.** Manacher's algorithm computes, for every center, the length of the longest
palindrome around it in linear total time. Interleave a separator so every palindrome has odd
length (`"aba"` becomes `"^#a#b#a#$"`); then `radius[i]` equals the length of the longest palindrome
of `s` centered at position `i` of the transformed string. Inside a known palindrome reaching
`right`, the mirror image of `i` already tells us a lower bound on `radius[i]`, so only characters
beyond `right` are ever compared.

A center whose longest palindrome has length `L` also hosts the shorter palindromes of lengths
`L - 2`, `L - 4`, ..., so it contributes `(L + 1) // 2` palindromes.

**Algorithm.**

1. Build `t = "^#" + "#".join(s) + "#$"` and compute `radius` with Manacher's expansion.
2. Return the sum of `(radius[i] + 1) // 2` over all positions.

```python
class Solution:
    def countSubstrings(self, s: str) -> int:
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
        return sum((r + 1) // 2 for r in radius)
```

- **Time:** `O(n)` — the right boundary only moves forward.
- **Space:** `O(n)`.
