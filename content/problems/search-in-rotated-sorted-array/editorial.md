## Approach 1: Linear Scan

Check every index.

```python
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        for i, x in enumerate(nums):
            if x == target:
                return i
        return -1
```

- **Time:** `O(n)` — correct, but it ignores the required `O(log n)` bound.
- **Space:** `O(1)`.

## Approach 2: Find the Pivot, Then Binary Search

**Intuition.** First locate the smallest element with a binary search (the technique from *Find
Minimum in Rotated Sorted Array*). That index splits the array into two sorted runs; the target
can only be in one of them, which an ordinary binary search handles.

```python
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        n = len(nums)
        lo, hi = 0, n - 1
        while lo < hi:
            mid = (lo + hi) // 2
            if nums[mid] > nums[hi]:
                lo = mid + 1
            else:
                hi = mid
        pivot = lo  # index of the minimum
        if pivot > 0 and target >= nums[0]:
            lo, hi = 0, pivot - 1
        else:
            lo, hi = pivot, n - 1
        i = bisect_left(nums, target, lo, hi + 1)
        return i if i <= hi and nums[i] == target else -1
```

- **Time:** `O(log n)` — two binary searches.
- **Space:** `O(1)`.

## Approach 3: One-Pass Binary Search (Optimal)

**Intuition.** Split the current range `[lo, hi]` at `mid`. Because there is at most one "drop"
in the array, at least one of `[lo, mid]` and `[mid, hi]` is sorted, and `nums[lo] <= nums[mid]`
tells us which. For the sorted half we can check with two comparisons whether the target lies
within its value range; if it does, we keep that half, otherwise we keep the other one.

**Algorithm.**

1. `lo, hi = 0, n - 1`.
2. While `lo <= hi`:
   - `mid = (lo + hi) // 2`; if `nums[mid] == target`, return `mid`.
   - If `nums[lo] <= nums[mid]` (left half sorted): if `nums[lo] <= target < nums[mid]`, set
     `hi = mid - 1`, else `lo = mid + 1`.
   - Otherwise (right half sorted): if `nums[mid] < target <= nums[hi]`, set `lo = mid + 1`,
     else `hi = mid - 1`.
3. Return `-1`.

The `<=` in `nums[lo] <= nums[mid]` matters: when `lo == mid` the one-element left half is
trivially sorted.

```python
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        lo, hi = 0, len(nums) - 1
        while lo <= hi:
            mid = (lo + hi) // 2
            if nums[mid] == target:
                return mid
            if nums[lo] <= nums[mid]:
                if nums[lo] <= target < nums[mid]:
                    hi = mid - 1
                else:
                    lo = mid + 1
            else:
                if nums[mid] < target <= nums[hi]:
                    lo = mid + 1
                else:
                    hi = mid - 1
        return -1
```

- **Time:** `O(log n)`.
- **Space:** `O(1)`.
