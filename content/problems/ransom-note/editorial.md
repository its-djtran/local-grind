## Approach 1: Remove Letters One at a Time

**Intuition.** Simulate the cutting directly: for every letter of the note, find a matching letter
in what remains of the magazine and remove it. If some letter cannot be found, the note is
impossible.

```python
class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        remaining = list(magazine)
        for ch in ransomNote:
            if ch not in remaining:
                return False
            remaining.remove(ch)
        return True
```

- **Time:** `O(n * m)` where `n = len(ransomNote)` and `m = len(magazine)` — every search and removal
  can take `O(m)`.
- **Space:** `O(m)` for the mutable copy.

## Approach 2: Sort Both Strings

**Intuition.** After sorting, both strings list their letters in the same order, so a two-pointer
merge can match each note letter with the next unused equal letter in the magazine.

```python
class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        note, mag = sorted(ransomNote), sorted(magazine)
        j = 0
        for ch in note:
            while j < len(mag) and mag[j] < ch:
                j += 1
            if j == len(mag) or mag[j] != ch:
                return False
            j += 1
        return True
```

- **Time:** `O(n log n + m log m)` for the sorts.
- **Space:** `O(n + m)` for the sorted copies.

## Approach 3: Letter Counts (Optimal)

**Intuition.** Only the *number* of each letter matters. Count how many of each letter the magazine
offers, then spend one unit of the right letter for every character of the note. The moment a
letter runs out, the answer is `false`. Since there are only 26 lowercase letters, an array of 26
counters suffices.

**Algorithm.**

1. If the note is longer than the magazine, return `false` immediately.
2. Count each letter of `magazine` into `counts[26]`.
3. For each letter of `ransomNote`: if its count is `0`, return `false`; otherwise decrement it.
4. Return `true`.

```python
class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        if len(ransomNote) > len(magazine):
            return False
        counts = [0] * 26
        for ch in magazine:
            counts[ord(ch) - 97] += 1
        for ch in ransomNote:
            i = ord(ch) - 97
            if counts[i] == 0:
                return False
            counts[i] -= 1
        return True
```

- **Time:** `O(n + m)` — one pass over each string.
- **Space:** `O(1)` — the alphabet has a fixed size of 26.

In Python, `Counter(ransomNote) <= Counter(magazine)` (Python 3.10+) or
`not Counter(ransomNote) - Counter(magazine)` expresses the same idea in a single line.
