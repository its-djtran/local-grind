You are given a string `s` holding a valid arithmetic expression made of non-negative integers, the operators `'+'` and `'-'`, parentheses `'('` and `')'`, and spaces. Evaluate the expression and return its value.

Spaces may appear anywhere between tokens and carry no meaning. A minus sign can also be **unary**, negating the number or parenthesized group that follows it, as in `"-1"` or `"-(2 + 3)"`. A plus sign is never unary, so inputs such as `"+1"` or `"+(2 + 3)"` will not appear, and no two operators ever appear next to each other.

You may **not** use any built-in facility that evaluates a string as an expression, such as `eval()`.

<!-- examples -->

**Constraints:**

- `1 <= s.length <= 3 * 10^5`
- `s` contains only digits, `'+'`, `'-'`, `'('`, `')'` and `' '`.
- `s` is a valid expression.
- `'+'` is never used as a unary operator.
- `'-'` may be used as a unary operator.
- No two operators are consecutive.
- Every number in `s`, and every intermediate result of the computation, fits in a signed 32-bit integer.
