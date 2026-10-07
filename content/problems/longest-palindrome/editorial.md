Since the letters can be rearranged freely, the only question is how many of each letter we can
use. In a palindrome, every character except possibly the middle one has a mirror image at the
same distance from the other end. So a palindrome consists of pairs of equal letters, plus at most
one extra letter in the center.

## Approach 1: Count Letters (Greedy)

**Intuition.** A letter that appears `c` times can contribute `c // 2` pairs, that is
`2 * (c // 2)` characters. Using every pair is always best. If any letter has an odd count, one of
its leftover copies can go in the middle for one more character.

**Algorithm.**

1. Count each character.
2. Sum `2 * (c // 2)` over all counts.
3. If any count is odd, add `1`.

```python
class Solution:
    def longestPalindrome(self, s: str) -> int:
        counts = Counter(s)
        length = sum(c // 2 * 2 for c in counts.values())
        if any(c % 2 for c in counts.values()):
            length += 1
        return length
```

- **Time:** `O(n)`.
- **Space:** `O(k)` where `k <= 52` is the number of distinct letters, so `O(1)`.

## Approach 2: Track Unpaired Letters (Optimal)

**Intuition.** Instead of counting, scan once and keep the set of letters whose copies cannot
yet be paired. Seeing a letter that is already in the set completes a pair (remove it); otherwise
it waits for a partner (add it). At the end, every character outside the set belongs to a pair,
and if the set is non-empty, one of its letters can occupy the center.

**Algorithm.**

1. Maintain a set `unpaired`; toggle each character in or out of it.
2. Return `len(s) - len(unpaired)`, plus `1` if `unpaired` is non-empty.

```python
class Solution:
    def longestPalindrome(self, s: str) -> int:
        unpaired = set()
        for ch in s:
            if ch in unpaired:
                unpaired.remove(ch)
            else:
                unpaired.add(ch)
        return len(s) - len(unpaired) + (1 if unpaired else 0)
```

- **Time:** `O(n)` — one pass.
- **Space:** `O(1)` — at most 52 distinct letters can be in the set.
