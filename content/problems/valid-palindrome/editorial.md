## Approach 1: Clean, Then Compare With the Reverse

**Intuition.** Do exactly what the definition says: build the filtered, lowercased string and
check whether it equals its own reverse.

```python
class Solution:
    def isPalindrome(self, s: str) -> bool:
        cleaned = [ch.lower() for ch in s if ch.isalnum()]
        return cleaned == cleaned[::-1]
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the cleaned copy and its reverse.

## Approach 2: Two Pointers (Optimal)

**Intuition.** The comparison in Approach 1 pairs the first kept character with the last, the
second with the second-to-last, and so on. Two indices walking inward from both ends can make the
same comparisons directly on `s`, skipping ignored characters on the fly, without building any
copy.

**Algorithm.**

1. Set `i = 0` and `j = len(s) - 1`.
2. While `i < j`:
   - Advance `i` while `i < j` and `s[i]` is not a letter or digit; retreat `j` likewise.
   - If `s[i].lower() != s[j].lower()`, return `false`.
   - Move both indices one step inward.
3. Return `true`.

Note that `isalnum` keeps digits as well as letters; filtering with `isalpha` would wrongly treat
`"0P"` as a palindrome.

```python
class Solution:
    def isPalindrome(self, s: str) -> bool:
        i, j = 0, len(s) - 1
        while i < j:
            while i < j and not s[i].isalnum():
                i += 1
            while i < j and not s[j].isalnum():
                j -= 1
            if s[i].lower() != s[j].lower():
                return False
            i += 1
            j -= 1
        return True
```

- **Time:** `O(n)` — each index moves across the string at most once.
- **Space:** `O(1)`.
