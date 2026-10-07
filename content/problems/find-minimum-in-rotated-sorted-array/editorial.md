## Approach 1: Linear Scan

Look at every element and keep the smallest — or, equivalently, find the one place where a value
is smaller than its predecessor.

```python
class Solution:
    def findMin(self, nums: List[int]) -> int:
        for i in range(1, len(nums)):
            if nums[i] < nums[i - 1]:
                return nums[i]
        return nums[0]
```

- **Time:** `O(n)` — correct, but it does not meet the required `O(log n)`.
- **Space:** `O(1)`.

## Approach 2: Binary Search Against the Last Element (Optimal)

**Intuition.** The rotated array consists of a "high" sorted run followed by a "low" sorted run
(either may be empty if the rotation brought the array back to sorted order). Every element of the
high run is larger than the last element of the array, and every element of the low run is at most
the last element. The minimum is the first element of the low run, so we binary search for the
boundary.

**Algorithm.**

1. `lo, hi = 0, n - 1`.
2. While `lo < hi`:
   - `mid = (lo + hi) // 2`.
   - If `nums[mid] > nums[hi]`, `mid` is in the high run, so the minimum is to its right:
     `lo = mid + 1`.
   - Otherwise `mid` is in the low run, so the minimum is `mid` or to its left: `hi = mid`.
3. Return `nums[lo]`.

Comparing with `nums[hi]` rather than `nums[lo]` handles the unrotated case automatically, and
distinct values guarantee `nums[mid] != nums[hi]` whenever `mid < hi`.

```python
class Solution:
    def findMin(self, nums: List[int]) -> int:
        lo, hi = 0, len(nums) - 1
        while lo < hi:
            mid = (lo + hi) // 2
            if nums[mid] > nums[hi]:
                lo = mid + 1
            else:
                hi = mid
        return nums[lo]
```

- **Time:** `O(log n)` — the range halves each step.
- **Space:** `O(1)`.
