Call a window of `s` **good** if it contains each character at least as many times as `t`
does.

## Approach 1: Grow a Window From Every Start

**Intuition.** For each starting index, extend to the right while counting characters until the
window becomes good. That is the shortest good window starting there; the overall answer is
the shortest among all starts.

```python
class Solution:
    def minWindow(self, s: str, t: str) -> str:
        need = Counter(t)
        best = ""
        for i in range(len(s)):
            have = Counter()
            missing = len(t)
            for j in range(i, len(s)):
                have[s[j]] += 1
                if have[s[j]] <= need[s[j]]:
                    missing -= 1
                if missing == 0:
                    if not best or j - i + 1 < len(best):
                        best = s[i:j + 1]
                    break
        return best
```

- **Time:** `O(m^2)` — each start may scan to the end of `s` (for example when no answer
  exists).
- **Space:** `O(k)` for the counters, where `k` is the alphabet size (52 letters).

## Approach 2: Sliding Window With a Missing Counter (Optimal)

**Intuition.** Two facts let us reuse work between starting points:

- If `s[left..right]` is good, so is every window that contains it, so for a fixed `right` we
  want the *largest* `left` that keeps the window good.
- As `right` moves right, that best `left` never moves left.

So both pointers only ever move forward. To test "is the window good?" in constant time, keep
`need[ch]` = how many more copies of `ch` the window still requires (negative means surplus),
and `missing` = the total number of required characters not yet covered. The window is good
exactly when `missing == 0`.

**Algorithm.**

1. Initialise `need = Counter(t)` and `missing = len(t)`.
2. For each `right`, with `ch = s[right]`: if `need[ch] > 0` this character fills a gap, so
   decrement `missing`; then decrement `need[ch]`.
3. When `missing == 0`:
   - Advance `left` while `need[s[left]] < 0` (that character is surplus), incrementing its
     `need` as it leaves.
   - Record the window if it is the shortest so far.
   - Remove `s[left]` (a required character), which makes `missing = 1`, and advance `left` so
     the scan can look for the next good window.

```python
class Solution:
    def minWindow(self, s: str, t: str) -> str:
        need = Counter(t)
        missing = len(t)
        left = 0
        best_start, best_len = 0, len(s) + 1
        for right, ch in enumerate(s):
            if need[ch] > 0:
                missing -= 1
            need[ch] -= 1
            if missing == 0:
                # Drop surplus characters from the left; the window stays valid.
                while need[s[left]] < 0:
                    need[s[left]] += 1
                    left += 1
                if right - left + 1 < best_len:
                    best_start, best_len = left, right - left + 1
                # Give up the leftmost required character to look for the next window.
                need[s[left]] += 1
                missing += 1
                left += 1
        return "" if best_len > len(s) else s[best_start:best_start + best_len]
```

- **Time:** `O(m + n)` — building the counter is `O(n)`, and each pointer crosses `s` once.
- **Space:** `O(k)` for the counter.
