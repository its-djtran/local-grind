You are given an array of strings `tokens` describing an arithmetic expression in **Reverse Polish Notation** (postfix notation), where every operator comes after its two operands. For example, `3 4 +` means `3 + 4`, and `2 3 4 * +` means `2 + (3 * 4)`.

Evaluate the expression and return its value as an integer.

Details:

- The operators are `+`, `-`, `*` and `/`, all binary.
- Each operand is either an integer literal or the result of an earlier sub-expression.
- Integer division **truncates toward zero** (so `7 / -2` is `-3`).
- No expression divides by zero.
- The expression is always valid, and its value and every intermediate result fit in a signed 32-bit integer.

<!-- examples -->

**Constraints:**

- `1 <= tokens.length <= 10^4`
- Each `tokens[i]` is an operator (`"+"`, `"-"`, `"*"` or `"/"`) or an integer in the range `[-200, 200]`.
