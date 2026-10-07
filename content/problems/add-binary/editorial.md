## Approach 1: Convert to Integers

**Intuition.** Python integers have arbitrary precision, so we can parse both strings as base-2
numbers, add them, and format the result back in binary. This is concise, but it sidesteps the
point of the exercise, and in languages with fixed-width integers it overflows for long inputs.

```python
class Solution:
    def addBinary(self, a: str, b: str) -> str:
        return bin(int(a, 2) + int(b, 2))[2:]
```

- **Time:** `O(n + m)` in practice (Python's base-2 conversions are linear).
- **Space:** `O(max(n, m))` for the integers and the result.

## Approach 2: Column Addition with a Carry (Optimal)

**Intuition.** Add the numbers exactly as you would by hand. Line up the strings at their right
ends and process one column at a time, moving left. A column's total is at most `1 + 1 + 1 = 3`:
its last bit is the digit to write down, and the remaining bit is the carry into the next column.
When one string runs out, treat its missing digits as `0`.

**Algorithm.**

1. Set pointers `i = len(a) - 1`, `j = len(b) - 1`, and `carry = 0`.
2. While `i >= 0`, `j >= 0`, or `carry` is non-zero:
   - `total = carry`, plus `a[i]` if `i >= 0`, plus `b[j]` if `j >= 0`; move the pointers left.
   - Append `total % 2` to a list of digits and set `carry = total // 2`.
3. The digits were produced least-significant first, so reverse them and join.

Continuing the loop while `carry` is set handles the final overflow column, as in
`"1" + "1" = "10"`. Appending to a list and reversing once avoids the quadratic cost of building
the string by repeated prepending.

```python
class Solution:
    def addBinary(self, a: str, b: str) -> str:
        i, j, carry = len(a) - 1, len(b) - 1, 0
        digits = []
        while i >= 0 or j >= 0 or carry:
            total = carry
            if i >= 0:
                total += ord(a[i]) - 48
                i -= 1
            if j >= 0:
                total += ord(b[j]) - 48
                j -= 1
            digits.append("1" if total & 1 else "0")
            carry = total >> 1
        return "".join(reversed(digits))
```

- **Time:** `O(max(n, m))` — one step per column.
- **Space:** `O(max(n, m))` for the output.
