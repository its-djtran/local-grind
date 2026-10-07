## Approach 1: Cross Off Letters One by One

**Intuition.** Take the letters of `t` as a pile. For each letter of `s`, find a matching
letter in the pile and remove it. If some letter cannot be found, or the pile is not empty at
the end, the strings are not anagrams.

```python
class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        pile = list(t)
        for ch in s:
            if ch not in pile:
                return False
            pile.remove(ch)
        return not pile
```

- **Time:** `O(n^2)` — every removal searches and shifts a list of up to `n` letters.
- **Space:** `O(n)` for the pile.

## Approach 2: Sort and Compare

**Intuition.** Sorting puts the letters of any word into a canonical order. Two strings are
anagrams exactly when their sorted forms are equal.

```python
class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        return sorted(s) == sorted(t)
```

- **Time:** `O(n log n)`.
- **Space:** `O(n)` for the sorted copies.

## Approach 3: Letter Counts (Optimal)

**Intuition.** We do not care about order at all, only about how many times each letter
occurs. With only 26 possible letters, an array of 26 counters captures everything: count up
for `s`, count down for `t`, and every counter must return to zero.

**Algorithm.**

1. If the lengths differ, return `False`.
2. Increment `count[ch]` for every letter of `s`.
3. Decrement `count[ch]` for every letter of `t`. If a counter goes negative, `t` has more of
   that letter than `s`, so return `False`. Because the lengths are equal, no counter can stay
   positive once none went negative.

```python
class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        count = [0] * 26
        for ch in s:
            count[ord(ch) - 97] += 1
        for ch in t:
            i = ord(ch) - 97
            count[i] -= 1
            if count[i] < 0:
                return False
        return True
```

- **Time:** `O(n)`.
- **Space:** `O(1)` — 26 counters.

**Follow-up (Unicode).** A fixed array of 26 no longer fits the alphabet; use a hash map
(`Counter(s) == Counter(t)`) instead. Strictly, Unicode text should also be normalized first
(for example with `unicodedata.normalize("NFC", ...)`) so that a character written as one code
point and the same character written as a base letter plus a combining accent compare equal.
