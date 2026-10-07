## Approach 1: Evaluate the Innermost Parentheses Repeatedly

**Intuition.** An expression without parentheses is easy: it is a sum of signed numbers. So find
a pair of parentheses with nothing nested inside, evaluate its contents as a flat sum, splice the
number back into the string in place of the group, and repeat until no parentheses are left.

The splice can produce two adjacent signs, as in `1-(-2)` becoming `1--2`, so the flat evaluator
lets several signs in a row multiply together.

```python
class Solution:
    def calculate(self, s: str) -> int:
        def flat(expr: str) -> int:
            total, sign, num, i = 0, 1, 0, 0
            while i < len(expr):
                ch = expr[i]
                if ch.isdigit():
                    num = num * 10 + int(ch)
                elif ch in "+-":
                    total += sign * num
                    num, sign = 0, 1 if ch == "+" else -1
                    while i + 1 < len(expr) and expr[i + 1] in "+-":
                        i += 1
                        if expr[i] == "-":
                            sign = -sign
                i += 1
            return total + sign * num

        s = s.replace(" ", "")
        while "(" in s:
            close = s.index(")")
            open_ = s.rindex("(", 0, close)
            s = s[:open_] + str(flat(s[open_ + 1:close])) + s[close + 1:]
        return flat(s)
```

- **Time:** `O(n * p)` where `p` is the number of parenthesis pairs; deeply nested input makes this
  `O(n^2)`.
- **Space:** `O(n)` for the rebuilt strings.

## Approach 2: Stack of Saved Totals (Optimal)

**Intuition.** Read the expression once, keeping a running `total` and the `sign` (+1 or -1) that
applies to the next operand. A number is folded in immediately as `total += sign * number`.

Parentheses interrupt that sum: everything up to the matching `)` must be computed first, then
added with the sign that preceded `(`. So on `(`, push the outer `total` and `sign` onto a stack and
start a fresh sum; on `)`, the fresh sum is complete, so pop the saved state and set
`total = saved_total + saved_sign * total`.

A unary minus needs no special case: at the start of the expression or right after `(`, the
running total is `0`, so `-x` is evaluated as `0 - x`.

**Algorithm.**

1. `total = 0`, `sign = 1`, `stack = []`, `num = 0`.
2. For each character:
   - digit: `num = num * 10 + digit`;
   - `+` or `-`: add `sign * num` to `total`, reset `num`, and set `sign` to `+1` or `-1`;
   - `(`: push `(total, sign)`, then reset `total = 0`, `sign = 1`;
   - `)`: add `sign * num` to `total` and reset `num`; pop `(prev, prev_sign)` and set
     `total = prev + prev_sign * total`;
   - spaces are skipped.
3. Return `total + sign * num`.

```python
class Solution:
    def calculate(self, s: str) -> int:
        total, sign, num = 0, 1, 0
        stack = []
        for ch in s:
            if ch.isdigit():
                num = num * 10 + ord(ch) - 48
            elif ch == "+" or ch == "-":
                total += sign * num
                num = 0
                sign = 1 if ch == "+" else -1
            elif ch == "(":
                stack.append((total, sign))
                total, sign = 0, 1
            elif ch == ")":
                total += sign * num
                num = 0
                prev, prev_sign = stack.pop()
                total = prev + prev_sign * total
        return total + sign * num
```

- **Time:** `O(n)` — one pass, constant work per character.
- **Space:** `O(n)` for the stack in the worst case (deep nesting).

A recursive-descent parser (a function that evaluates up to the matching `)` and calls itself on
each `(`) is equally linear, but on very deep inputs it needs a recursion depth equal to the
nesting depth.
