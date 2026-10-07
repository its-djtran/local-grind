## Approach 1: Check Every Substring

**Intuition.** The most direct idea: enumerate all substrings, test each one for duplicate
characters, and keep the longest that passes.

```python
class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        best = 0
        for i in range(len(s)):
            for j in range(i + 1, len(s) + 1):
                if len(set(s[i:j])) == j - i:
                    best = max(best, j - i)
        return best
```

- **Time:** `O(n^3)` — `O(n^2)` substrings, each checked in `O(n)`.
- **Space:** `O(k)` for the set, where `k` is the alphabet size.

## Approach 2: Extend From Each Start

**Intuition.** For a fixed starting index, keep adding characters until one repeats; at that
point no longer substring from this start can be valid, so stop. This avoids rechecking the
same characters over and over within a start.

```python
class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        best = 0
        for i in range(len(s)):
            seen = set()
            for j in range(i, len(s)):
                if s[j] in seen:
                    break
                seen.add(s[j])
            best = max(best, len(seen))
        return best
```

- **Time:** `O(n * min(n, k))` — each start scans at most `k` characters before a repeat is
  forced. With ~95 printable characters and `n = 5 * 10^4` this is still millions of steps.
- **Space:** `O(k)`.

## Approach 3: Sliding Window With Last-Seen Indices (Optimal)

**Intuition.** Approach 2 throws away everything it learned when it moves to the next start.
Instead keep one window `[left, right]` of distinct characters and slide it. When the new
character `s[right]` already occurs inside the window at index `p`, every window that starts at
or before `p` would contain it twice, so `left` can jump straight to `p + 1`. A map from each
character to its most recent index gives `p` in constant time.

**Algorithm.**

1. Keep `last[ch]` = most recent index of `ch`, and `left = 0`.
2. For each `right`, with `ch = s[right]`:
   - If `ch` was seen at an index `>= left`, move `left` to `last[ch] + 1`. (An older
     occurrence before `left` is outside the window and must be ignored.)
   - Update `last[ch] = right` and record the window length `right - left + 1`.

```python
class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        last = {}
        left = best = 0
        for right, ch in enumerate(s):
            if last.get(ch, -1) >= left:
                left = last[ch] + 1
            last[ch] = right
            if right - left + 1 > best:
                best = right - left + 1
        return best
```

- **Time:** `O(n)` — each index is visited once.
- **Space:** `O(min(n, k))` for the map.
