Write `myAtoi(s)`, which turns the beginning of a string into a signed 32-bit integer, in the spirit of C's `atoi`. Process `s` from left to right as follows:

1. **Spaces:** skip any leading space characters `' '`.
2. **Sign:** if the next character is `'-'`, the result is negative; if it is `'+'`, the result is positive. Consume at most one such sign. With no sign, the result is positive.
3. **Digits:** read consecutive digit characters and form a number from them, ignoring leading zeros. Stop at the first character that is not a digit, or at the end of the string. Everything after that point is ignored. If no digits are read, the result is `0`.
4. **Clamping:** if the number lies outside the signed 32-bit range `[-2^31, 2^31 - 1]`, replace it with the nearest end of that range: values below `-2^31` become `-2^31`, and values above `2^31 - 1` become `2^31 - 1`.

Return the resulting integer.

<!-- examples -->

**Constraints:**

- `0 <= s.length <= 200`
- `s` contains only English letters (upper and lower case), digits, `' '`, `'+'`, `'-'` and `'.'`.
