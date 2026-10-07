## Approach 1: Check Every Window

**Intuition.** There are `n - m + 1` windows of length `m = len(p)` in `s`. For each one, test
whether it is an anagram of `p` by comparing letter counts (or sorted letters).

```python
class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        target = Counter(p)
        m = len(p)
        return [i for i in range(len(s) - m + 1) if Counter(s[i:i + m]) == target]
```

- **Time:** `O((n - m + 1) * m)` — roughly `O(n * m)`, which is very slow when `p` is about half
  the length of `s`.
- **Space:** `O(m)` for the slice.

## Approach 2: Sliding Window of Counts

**Intuition.** Neighbouring windows share all but two characters. Keep the 26 letter counts of
the current window; sliding one step means one increment (the entering letter) and one
decrement (the leaving letter). Then compare the 26 counts with those of `p`.

```python
class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        m = len(p)
        if m > len(s):
            return []
        want, have = [0] * 26, [0] * 26
        for ch in p:
            want[ord(ch) - 97] += 1
        out = []
        for i, ch in enumerate(s):
            have[ord(ch) - 97] += 1
            if i >= m:
                have[ord(s[i - m]) - 97] -= 1
            if i >= m - 1 and have == want:
                out.append(i - m + 1)
        return out
```

- **Time:** `O(26 * n)` — each step compares two arrays of 26 counts.
- **Space:** `O(1)`.

## Approach 3: Sliding Window With a Mismatch Counter (Optimal)

**Intuition.** Comparing all 26 counts at each step is wasteful when only two of them changed.
Store `diff[c]` = (count of `c` in the window) − (count of `c` in `p`), and `bad` = how many
letters have a non-zero `diff`. The window is an anagram exactly when `bad == 0`. When a count
changes, `bad` changes only if that letter's `diff` moves to or away from zero.

**Algorithm.**

1. If `len(p) > len(s)`, return `[]`.
2. Initialise `diff` from `p` (minus) and the first window (plus), and compute `bad`.
3. Slide the window: for the entering letter, `diff` goes up by one; for the leaving letter,
   it goes down by one. Before each change, adjust `bad`: moving away from zero adds one,
   arriving at zero subtracts one. (If the entering and leaving letters are the same, nothing
   changes.)
4. Record the start index whenever `bad == 0`.

```python
class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        m = len(p)
        if m > len(s):
            return []
        diff = [0] * 26  # window count minus p count, per letter
        for ch in p:
            diff[ord(ch) - 97] -= 1
        codes = [ord(ch) - 97 for ch in s]
        for c in codes[:m]:
            diff[c] += 1
        bad = sum(1 for d in diff if d != 0)  # letters whose counts disagree
        out = [0] if bad == 0 else []
        for i in range(m, len(codes)):
            add, rem = codes[i], codes[i - m]
            if add != rem:
                d = diff[add]
                bad += (d == 0) - (d == -1)
                diff[add] = d + 1
                d = diff[rem]
                bad += (d == 0) - (d == 1)
                diff[rem] = d - 1
            if bad == 0:
                out.append(i - m + 1)
        return out
```

- **Time:** `O(n + m)`.
- **Space:** `O(1)` besides the output (the list of letter codes is a convenience and can be
  dropped by calling `ord` inline).
