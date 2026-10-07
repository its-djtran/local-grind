A substring can be turned into one repeated letter with `length - maxCount` changes, where
`maxCount` is the number of times its most frequent letter appears: keep that letter, overwrite
the rest. So the question becomes: what is the longest substring with
`length - maxCount <= k`?

## Approach 1: Grow From Every Start

**Intuition.** Fix a starting index and extend the substring one letter at a time while
tracking letter counts and the largest count. As soon as the cost exceeds `k`, any longer
substring from this start is also too costly, so stop.

```python
class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        best = 0
        for i in range(len(s)):
            count = [0] * 26
            maxf = 0
            for j in range(i, len(s)):
                c = ord(s[j]) - 65
                count[c] += 1
                maxf = max(maxf, count[c])
                if j - i + 1 - maxf > k:
                    break
                best = max(best, j - i + 1)
        return best
```

- **Time:** `O(n^2)` in the worst case (for example, when `k` is large every start scans most of
  the string).
- **Space:** `O(1)` — 26 counters.

## Approach 2: Sliding Window per Target Letter

**Intuition.** Pick the letter `t` that the final run will consist of. Then the cost of a window
is just the number of letters in it that are not `t`, which only grows when the window grows
and only shrinks when it shrinks: a textbook sliding window. Repeat for all 26 letters.

```python
class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        best = 0
        for t in set(s):
            left = others = 0
            for right, ch in enumerate(s):
                if ch != t:
                    others += 1
                while others > k:
                    if s[left] != t:
                        others -= 1
                    left += 1
                best = max(best, right - left + 1)
        return best
```

- **Time:** `O(26 * n)`.
- **Space:** `O(1)`.

## Approach 3: Single Sliding Window (Optimal)

**Intuition.** Use one window and 26 counters, with `maxf` holding the largest letter count
ever seen in a window. The key observation is that we only care about windows *longer* than the
best found so far. A longer valid window needs a larger `maxf`, so when the window becomes
invalid we do not need to shrink it at all: we slide it one step to the right (add one letter on
the right, drop one on the left), keeping its length. `maxf` may then be stale (too high for the
current window), but a stale value can only stop us from shrinking, never produce an answer that
is too large, because the window length only grows when a genuinely larger `maxf` is reached.

**Algorithm.**

1. For each `right`, add `s[right]` to the counts and update `maxf`.
2. If `window length - maxf > k`, remove `s[left]` from the counts and advance `left` by one.
3. The window length never decreases, so at the end it equals the answer: `n - left`.

```python
class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        count = [0] * 26
        left = maxf = 0
        for right in range(len(s)):
            c = ord(s[right]) - 65
            count[c] += 1
            if count[c] > maxf:
                maxf = count[c]
            # The window never shrinks: if it is too costly, slide it right by one instead.
            if right - left + 1 - maxf > k:
                count[ord(s[left]) - 65] -= 1
                left += 1
        return len(s) - left
```

- **Time:** `O(n)`.
- **Space:** `O(1)`.
