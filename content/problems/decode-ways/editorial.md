## Approach 1: Brute-Force Recursion

**Intuition.** Look at the start of the remaining string. The first group is either one digit
(valid if it is not `'0'`) or two digits (valid if they form `10` through `26`). Count the
decodings of what remains after each valid choice and add them up.

```python
class Solution:
    def numDecodings(self, s: str) -> int:
        def count(i: int) -> int:
            if i == len(s):
                return 1
            if s[i] == "0":
                return 0
            total = count(i + 1)
            if i + 1 < len(s) and 10 <= int(s[i:i + 2]) <= 26:
                total += count(i + 2)
            return total

        return count(0)
```

- **Time:** `O(phi^n)` in the worst case (for example a long run of `1`s) — the call tree has a
  leaf for every decoding, and there can be billions of them.
- **Space:** `O(n)` for the recursion.

## Approach 2: Memoized Recursion

**Intuition.** `count(i)` depends only on `i`. With a cache, each of the `n + 1` suffixes is
counted once.

```python
class Solution:
    def numDecodings(self, s: str) -> int:
        @lru_cache(maxsize=None)
        def count(i: int) -> int:
            if i == len(s):
                return 1
            if s[i] == "0":
                return 0
            total = count(i + 1)
            if i + 1 < len(s) and 10 <= int(s[i:i + 2]) <= 26:
                total += count(i + 2)
            return total

        return count(0)
```

- **Time:** `O(n)`.
- **Space:** `O(n)`.

## Approach 3: Bottom-Up with Two Variables (Optimal)

**Intuition.** Let `dp[i]` be the number of decodings of the prefix `s[:i]`, with `dp[0] = 1`.
The final group of a decoding of `s[:i]` is either:

- the single digit `s[i - 1]`, allowed when it is not `'0'` — contributes `dp[i - 1]`;
- the pair `s[i - 2 : i]`, allowed when it is between `"10"` and `"26"` — contributes `dp[i - 2]`.

Since `dp[i]` only needs the previous two entries, keep them in two variables.

**Algorithm.**

1. `prev = 1` (that is `dp[i - 2]`) and `cur = 1` if `s[0] != '0'` else `0` (that is `dp[i - 1]`).
2. For each `i` from `1` to `n - 1`, compute `nxt = 0`; add `cur` if `s[i] != '0'`; add `prev` if
   `s[i - 1] == '1'`, or `s[i - 1] == '2'` and `s[i] <= '6'`. Then shift: `prev, cur = cur, nxt`.
3. Return `cur`.

Once `cur` and `prev` are both `0`, every later value is `0` too, which correctly handles strings
like `"100"` or `"30"`.

```python
class Solution:
    def numDecodings(self, s: str) -> int:
        prev, cur = 1, (0 if s[0] == "0" else 1)
        for i in range(1, len(s)):
            nxt = 0
            if s[i] != "0":
                nxt += cur
            if s[i - 1] == "1" or (s[i - 1] == "2" and s[i] <= "6"):
                nxt += prev
            prev, cur = cur, nxt
        return cur
```

- **Time:** `O(n)`.
- **Space:** `O(1)`.
