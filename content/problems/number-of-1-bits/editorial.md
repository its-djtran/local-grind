## Approach 1: Check Every Bit

**Intuition.** The lowest bit of `n` is `n & 1`. Shifting `n` right by one exposes the next bit,
so we can look at every bit in turn and add up the ones.

**Algorithm.**

1. Set `count = 0`.
2. While `n > 0`: add `n & 1` to `count`, then shift `n >>= 1`.
3. Return `count`.

```python
class Solution:
    def hammingWeight(self, n: int) -> int:
        count = 0
        while n:
            count += n & 1
            n >>= 1
        return count
```

- **Time:** `O(b)` where `b` is the number of bits (at most 31 here), regardless of how many are set.
- **Space:** `O(1)`.

## Approach 2: Clear the Lowest Set Bit (Optimal)

**Intuition.** Subtracting `1` from `n` flips its lowest `1` bit to `0` and turns every `0` below
it into `1`. ANDing with the original therefore erases exactly that lowest set bit and nothing
else. For example, `n = 10100` gives `n - 1 = 10011` and `n & (n - 1) = 10000`. Repeating this
until `n` becomes `0` takes one step per set bit.

**Algorithm.**

1. Set `count = 0`.
2. While `n` is non-zero: `n &= n - 1`, then increment `count`.
3. Return `count`.

```python
class Solution:
    def hammingWeight(self, n: int) -> int:
        count = 0
        while n:
            n &= n - 1
            count += 1
        return count
```

- **Time:** `O(k)` where `k` is the number of `1` bits — never more than Approach 1.
- **Space:** `O(1)`.

## Follow-up: Many Calls

If the function is called millions of times, precompute the bit counts of all 256 byte values
once (`table[i] = table[i >> 1] + (i & 1)`), then answer each query by splitting `n` into four
bytes and adding four table lookups. Each call becomes a constant number of operations. Another
constant-time option is the "SWAR" trick, which adds neighbouring bit counts in parallel:

```python
def popcount32(n: int) -> int:
    n = n - ((n >> 1) & 0x55555555)
    n = (n & 0x33333333) + ((n >> 2) & 0x33333333)
    n = (n + (n >> 4)) & 0x0F0F0F0F
    return ((n * 0x01010101) & 0xFFFFFFFF) >> 24
```
