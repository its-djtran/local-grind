## Approach 1: Repeatedly Delete Adjacent Pairs

**Intuition.** In any balanced string that is not empty, some opener is immediately followed by
its own closer (the innermost pair). Deleting that pair leaves a string that is balanced exactly
when the original was. So keep deleting `"()"`, `"[]"` and `"{}"` until nothing changes; the string
was balanced if and only if it shrinks to empty.

```python
class Solution:
    def isValid(self, s: str) -> bool:
        while True:
            shorter = s.replace("()", "").replace("[]", "").replace("{}", "")
            if shorter == s:
                return s == ""
            s = shorter
```

- **Time:** `O(n^2)` — deeply nested input such as `"((((...))))"` loses only one pair per
  round, and each round rescans the string.
- **Space:** `O(n)` for the rebuilt strings.

## Approach 2: Stack (Optimal)

**Intuition.** When a closing bracket arrives, the only opener it can legally close is the most
recent one that has not been closed yet. "Most recent unfinished thing" is exactly what a stack
tracks.

**Algorithm.**

1. Map each closer to its opener: `')' -> '('`, `']' -> '['`, `'}' -> '{'`.
2. Scan `s`. Push every opener. For a closer, fail if the stack is empty or its top is not the
   matching opener; otherwise pop.
3. After the scan, the string is balanced exactly when the stack is empty (no unclosed openers).

A quick extra check: a string of odd length can never be balanced.

```python
class Solution:
    def isValid(self, s: str) -> bool:
        pairs = {")": "(", "]": "[", "}": "{"}
        stack = []
        for ch in s:
            if ch in pairs:
                if not stack or stack.pop() != pairs[ch]:
                    return False
            else:
                stack.append(ch)
        return not stack
```

- **Time:** `O(n)` — one pass with constant work per character.
- **Space:** `O(n)` in the worst case, when the string is all openers.
