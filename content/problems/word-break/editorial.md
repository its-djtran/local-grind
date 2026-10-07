## Approach 1: Brute-Force Recursion

**Intuition.** The first piece of a valid split is some dictionary word that is a prefix of `s`.
Try every such prefix and recursively ask whether the remainder can be split.

```python
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        words = set(wordDict)

        def can(start: int) -> bool:
            if start == len(s):
                return True
            for end in range(start + 1, len(s) + 1):
                if s[start:end] in words and can(end):
                    return True
            return False

        return can(0)
```

- **Time:** `O(2^n)` in the worst case. With `s = "aaaa...ab"` and the words `"a"`, `"aa"`,
  `"aaa"`, ..., every way of splitting the run of `a`s is explored before the `b` dooms them all.
- **Space:** `O(n)` for the recursion.

## Approach 2: Memoized Recursion

**Intuition.** Whether the suffix starting at `start` is splittable does not depend on how we got
there, so there are only `n + 1` distinct questions. Cache each answer.

```python
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        words = set(wordDict)

        @lru_cache(maxsize=None)
        def can(start: int) -> bool:
            if start == len(s):
                return True
            for end in range(start + 1, len(s) + 1):
                if s[start:end] in words and can(end):
                    return True
            return False

        return can(0)
```

- **Time:** `O(n^3)` — `n` states, each trying up to `n` end points, and each slice and hash costs
  up to `O(n)`.
- **Space:** `O(n)` for the cache and recursion.

## Approach 3: Bottom-Up DP with Bounded Word Length (Optimal)

**Intuition.** Let `ok[i]` say whether the prefix `s[:i]` can be split. The empty prefix can
(`ok[0] = True`), and `ok[i]` is true exactly when some earlier split point `j` has `ok[j]` true
and `s[j:i]` is a word. Since no word is longer than the longest dictionary word `L` (at most 20),
`j` only needs to range over `i - L .. i - 1`.

**Algorithm.**

1. Put the words in a hash set and let `L` be the longest word length.
2. `ok = [True] + [False] * n`.
3. For each `i` from `1` to `n`, for each `j` from `max(0, i - L)` to `i - 1`: if `ok[j]` and
   `s[j:i]` is in the set, mark `ok[i] = True` and stop checking this `i`.
4. Return `ok[n]`.

```python
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        words = set(wordDict)
        longest = max(map(len, words))
        n = len(s)
        ok = [True] + [False] * n
        for i in range(1, n + 1):
            for j in range(max(0, i - longest), i):
                if ok[j] and s[j:i] in words:
                    ok[i] = True
                    break
        return ok[n]
```

- **Time:** `O(n * L^2 + D)` — `n * L` candidate pieces, each sliced and hashed in `O(L)`, plus
  `O(D)` to build the set from the dictionary's total length `D`.
- **Space:** `O(n + D)` for the table and the set.

A trie over the dictionary is a common alternative: from each reachable position, walk the trie
character by character and mark every position where a word ends. It has the same `O(n * L)`
character steps without hashing slices.
