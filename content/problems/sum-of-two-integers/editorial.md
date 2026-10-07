Both approaches treat the numbers as 32-bit two's-complement patterns, which is how hardware adds
signed integers. Python integers have unlimited width, so we keep only the low 32 bits with a mask
and convert the result back to a signed value at the end: a pattern whose bit 31 is set represents
`pattern - 2^32`, which can be written without subtraction as `~(pattern ^ 0xFFFFFFFF)`.

## Approach 1: Bit-by-Bit Ripple Carry

**Intuition.** Add the two numbers exactly as you would on paper, one binary column at a time from
the least significant bit, carrying a `1` into the next column when needed.

**Algorithm.**

1. Mask both inputs to 32 bits and set `carry = 0`, `result = 0`.
2. For each bit position `i` from `0` to `31`:
   - Let `x` and `y` be bit `i` of `a` and `b`.
   - The column digit is `x ^ y ^ carry`; place it at position `i` of `result`.
   - The new carry is `1` when at least two of `x`, `y`, `carry` are `1`:
     `(x & y) | (x & carry) | (y & carry)`.
3. Convert `result` from a 32-bit pattern to a signed integer.

```python
class Solution:
    def getSum(self, a: int, b: int) -> int:
        mask = 0xFFFFFFFF
        a &= mask
        b &= mask
        result = carry = 0
        for i in range(32):
            x = (a >> i) & 1
            y = (b >> i) & 1
            result |= (x ^ y ^ carry) << i
            carry = (x & y) | (x & carry) | (y & carry)
        return result if result <= 0x7FFFFFFF else ~(result ^ mask)
```

- **Time:** `O(32)` = `O(1)` — one pass over a fixed number of bits.
- **Space:** `O(1)`.

## Approach 2: XOR and Carry Loop (Optimal)

**Intuition.** Instead of one column at a time, handle all columns at once. `a ^ b` is the sum of
every column *ignoring* carries, and `(a & b) << 1` is exactly the set of carries, already moved
into the columns they flow into. Their true sum equals the original sum, so we can repeat the
step on this new pair. Each round pushes the lowest carry at least one position to the left, so
after at most 32 rounds (with masking) the carry disappears and `a` holds the answer.

**Algorithm.**

1. Mask `a` and `b` to 32 bits.
2. While `b` is non-zero: set `a, b = (a ^ b) & mask, ((a & b) << 1) & mask`.
3. If bit 31 of `a` is set, return `~(a ^ mask)`; otherwise return `a`.

Without the mask, adding a negative and a positive number in Python never terminates: the carry
keeps marching left forever because Python's negative integers have infinitely many leading ones.

```python
class Solution:
    def getSum(self, a: int, b: int) -> int:
        mask = 0xFFFFFFFF
        a &= mask
        b &= mask
        while b:
            a, b = (a ^ b) & mask, ((a & b) << 1) & mask
        return a if a <= 0x7FFFFFFF else ~(a ^ mask)
```

- **Time:** `O(1)` — at most 32 iterations for 32-bit integers.
- **Space:** `O(1)`.
