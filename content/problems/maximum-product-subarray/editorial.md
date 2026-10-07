## Approach 1: Brute Force

Fix each start index and extend to the right, multiplying as you go.

```python
class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        best = nums[0]
        for i in range(len(nums)):
            p = 1
            for j in range(i, len(nums)):
                p *= nums[j]
                best = max(best, p)
        return best
```

- **Time:** `O(n^2)` — about `2 * 10^8` multiplications at the maximum size.
- **Space:** `O(1)`.

## Approach 2: Track Both Extremes (Optimal)

**Intuition.** With sums, a bad prefix is simply dropped (Kadane's algorithm). With products a
"bad" prefix can be very useful: the most negative product so far turns into the most positive
one as soon as another negative number arrives. So for the subarrays ending at each index we
remember two values — the largest product `hi` and the smallest product `lo`.

**Algorithm.**

1. Start with `hi = lo = best = nums[0]`.
2. For each next value `x`, the best and worst products ending here are among
   `x`, `hi * x`, and `lo * x`. Compute both from the *old* `hi` and `lo`.
3. Update `best = max(best, hi)`.

A zero makes both extremes `0`, which correctly restarts the search on the next element because
`x` alone is always a candidate.

```python
class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        hi = lo = best = nums[0]
        for x in nums[1:]:
            a, b = hi * x, lo * x
            hi = max(x, a, b)
            lo = min(x, a, b)
            if hi > best:
                best = hi
        return best
```

- **Time:** `O(n)`.
- **Space:** `O(1)`.

An equivalent trick: the answer is the largest prefix product or suffix product, restarting after
every zero, because the optimal subarray within a zero-free stretch always touches one of the
stretch's ends.
