Because `n` is even, its lowest bit is `0`, so the reversed value never has bit 31 set and always
fits in a signed 32-bit integer.

## Approach 1: String Reversal

**Intuition.** Format `n` as a 32-character binary string (the zero-padding is essential), reverse
the string, and parse it back.

```python
class Solution:
    def reverseBits(self, n: int) -> int:
        return int(format(n, "032b")[::-1], 2)
```

- **Time:** `O(32)` = `O(1)`.
- **Space:** `O(32)` for the temporary strings.

## Approach 2: Bit-by-Bit Shifting (Optimal)

**Intuition.** Reversing is like moving cards from the top of one pile to the top of another:
repeatedly take the lowest bit off `n` and append it to the low end of `result`. The first bit
taken (bit `0` of `n`) gets shifted left 31 more times and lands in position `31`; the last bit
taken (bit `31`) stays in position `0`.

**Algorithm.**

1. Set `result = 0`.
2. Repeat exactly 32 times:
   - `result = (result << 1) | (n & 1)`
   - `n >>= 1`
3. Return `result`.

Stopping early when `n` becomes `0` would be a bug: the remaining leading zeros of `n` still have
to be moved, which shifts everything already in `result` further left.

```python
class Solution:
    def reverseBits(self, n: int) -> int:
        result = 0
        for _ in range(32):
            result = (result << 1) | (n & 1)
            n >>= 1
        return result
```

- **Time:** `O(32)` = `O(1)`.
- **Space:** `O(1)`.

## Follow-up: Many Calls

Two common speed-ups:

- **Byte lookup table.** Precompute the reversal of all 256 byte values once. To reverse a 32-bit
  number, reverse each of its four bytes with the table and place them in the opposite order.
  Each call is then four lookups and a few shifts.
- **Divide and conquer.** Swap the two 16-bit halves, then the bytes inside each half, then the
  nibbles, pairs, and single bits, using masks. Five constant-time steps reverse all 32 bits:

```python
class Solution:
    def reverseBits(self, n: int) -> int:
        n = ((n >> 16) | (n << 16)) & 0xFFFFFFFF
        n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8)
        n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4)
        n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2)
        n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1)
        return n
```
