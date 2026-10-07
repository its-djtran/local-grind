Write `n` as an unsigned **32-bit** binary number, padding with leading zeros as needed. Reverse the order of those 32 bits, so the lowest bit becomes the highest and vice versa, and return the integer the reversed pattern represents.

For example, the bit at position `0` (the least significant bit) moves to position `31`, the bit at position `1` moves to position `30`, and so on.

<!-- examples -->

**Constraints:**

- `0 <= n <= 2^31 - 2`
- `n` is even.

**Follow-up:** If this function were called many times, how could you speed it up?
