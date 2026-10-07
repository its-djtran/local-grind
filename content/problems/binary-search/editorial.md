## Approach 1: Linear Scan

**Intuition.** Check every element in order and return the index where `target` appears. This
ignores the fact that the array is sorted, so it does not meet the `O(log n)` requirement.

```python
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        for i, x in enumerate(nums):
            if x == target:
                return i
        return -1
```

- **Time:** `O(n)`.
- **Space:** `O(1)`.

## Approach 2: Iterative Binary Search (Optimal)

**Intuition.** Because the array is sorted, one comparison with the middle element rules out half
of the remaining candidates: if `nums[mid] < target`, then every element at or before `mid` is
also too small; if `nums[mid] > target`, everything at or after `mid` is too large. Halving the
range each step leaves a single candidate after about `log2(n)` steps.

**Algorithm.**

1. Keep an inclusive search range `lo = 0`, `hi = n - 1`.
2. While `lo <= hi`:
   - `mid = (lo + hi) // 2`.
   - If `nums[mid] == target`, return `mid`.
   - If `nums[mid] < target`, the answer is to the right: `lo = mid + 1`.
   - Otherwise it is to the left: `hi = mid - 1`.
3. The range is empty, so `target` is absent: return `-1`.

Using `mid + 1` and `mid - 1` (rather than `mid`) guarantees the range shrinks on every iteration,
so the loop always terminates. In languages with fixed-width integers, compute
`mid = lo + (hi - lo) // 2` to avoid overflow; Python integers do not overflow.

```python
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        lo, hi = 0, len(nums) - 1
        while lo <= hi:
            mid = (lo + hi) // 2
            if nums[mid] == target:
                return mid
            if nums[mid] < target:
                lo = mid + 1
            else:
                hi = mid - 1
        return -1
```

- **Time:** `O(log n)` — the range halves on every iteration.
- **Space:** `O(1)`.

Python's standard library offers the same search: `i = bisect_left(nums, target)` finds the first
position whose value is at least `target`, and the answer is `i` if `i < len(nums)` and
`nums[i] == target`, otherwise `-1`.
