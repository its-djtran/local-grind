A message made of capital letters was turned into digits using the code `'A' -> "1"`, `'B' -> "2"`, ..., `'Z' -> "26"`, and the digits were written down with no separators.

To decode a digit string you must split it into groups that each spell a letter: every group is a number from `1` to `26` **written without a leading zero**. For example, `"11106"` can be split as `(1, 1, 10, 6)` for `"AAJF"` or as `(11, 10, 6)` for `"KJF"`, but the grouping `(1, 11, 06)` is not allowed because `"06"` is not a valid code (only `"6"` is).

Given a string `s` of digits, return the **number of different ways** to decode it. If no valid split exists, return `0`.

The tests are built so that the answer fits in a 32-bit signed integer.

<!-- examples -->

**Constraints:**

- `1 <= s.length <= 100`
- `s` contains only digits and may contain leading zeros.
