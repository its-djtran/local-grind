## Approach 1: Backtracking

**Intuition.** Build each answer one character at a time. Position `i` can hold any letter of
`digits[i]`; for each choice, recursively fill the remaining positions, and when every position is
filled, record the string.

```python
class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        keypad = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl",
                  "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}
        result, path = [], []

        def backtrack(i):
            if i == len(digits):
                result.append("".join(path))
                return
            for ch in keypad[digits[i]]:
                path.append(ch)
                backtrack(i + 1)
                path.pop()

        backtrack(0)
        return result
```

The empty input needs special care: without the early return, the recursion would record one
empty string and return `[""]` instead of `[]`.

- **Time:** `O(n * 4^n)` for `n` digits: at most `4^n` strings, each of length `n`.
- **Space:** `O(n)` for the recursion and the current path, excluding the output.

## Approach 2: Iterative Expansion (Optimal)

**Intuition.** The combinations for the first `k + 1` digits are the combinations for the first
`k` digits, each extended by every letter of digit `k + 1`. Grow the list digit by digit.

**Algorithm.**

1. If `digits` is empty, return `[]`.
2. Start with `combos = [""]`.
3. For each digit, replace `combos` with `[c + letter for c in combos for letter in keypad[digit]]`.

```python
class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        keypad = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl",
                  "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}
        combos = [""]
        for d in digits:
            combos = [c + letter for c in combos for letter in keypad[d]]
        return combos
```

- **Time:** `O(n * 4^n)`, proportional to the total length of the output.
- **Space:** `O(n * 4^n)` for the output.
