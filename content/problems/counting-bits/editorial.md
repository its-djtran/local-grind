## Approach 1: Count Each Number Independently

**Intuition.** Apply a single-number bit count to every `i` from `0` to `n`. Clearing the lowest
set bit with `x &= x - 1` takes one step per `1` bit.

```python
class Solution:
    def countBits(self, n: int) -> List[int]:
        ans = []
        for i in range(n + 1):
            x, c = i, 0
            while x:
                x &= x - 1
                c += 1
            ans.append(c)
        return ans
```

- **Time:** `O(n log n)` — each number has at most `log2(n) + 1` bits.
- **Space:** `O(1)` beyond the output.

## Approach 2: DP on the Lowest Set Bit

**Intuition.** `i & (i - 1)` is `i` with its lowest `1` bit removed. That number is smaller than
`i`, so its answer is already in the array, and `i` has exactly one more `1` bit.

```python
class Solution:
    def countBits(self, n: int) -> List[int]:
        ans = [0] * (n + 1)
        for i in range(1, n + 1):
            ans[i] = ans[i & (i - 1)] + 1
        return ans
```

- **Time:** `O(n)`.
- **Space:** `O(1)` beyond the output.

## Approach 3: DP on the Right Shift (Optimal)

**Intuition.** Writing `i` in binary and deleting its last digit gives `i >> 1` (that is, `i // 2`).
The two numbers share every bit except the last one, which is `i & 1`. So the count for `i` is the
count for `i >> 1`, plus one if `i` is odd. Because `i >> 1 < i` for every `i >= 1`, filling the
array from left to right always finds the value it needs.

**Algorithm.**

1. Create `ans = [0] * (n + 1)`; `ans[0] = 0` is the base case.
2. For `i` from `1` to `n`: `ans[i] = ans[i >> 1] + (i & 1)`.
3. Return `ans`.

```python
class Solution:
    def countBits(self, n: int) -> List[int]:
        ans = [0] * (n + 1)
        for i in range(1, n + 1):
            ans[i] = ans[i >> 1] + (i & 1)
        return ans
```

- **Time:** `O(n)` — one constant-time step per entry.
- **Space:** `O(1)` beyond the output array.
