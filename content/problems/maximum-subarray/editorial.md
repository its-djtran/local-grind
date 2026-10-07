## Approach 1: Brute Force

Fix each start index and extend the end one step at a time, keeping a running sum.

```python
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        best = nums[0]
        for i in range(len(nums)):
            s = 0
            for j in range(i, len(nums)):
                s += nums[j]
                best = max(best, s)
        return best
```

- **Time:** `O(n^2)` — about `5 * 10^9` steps at the maximum size.
- **Space:** `O(1)`.

## Approach 2: Divide and Conquer

**Intuition.** Cut the array in half. The best subarray lies entirely in the left half, entirely
in the right half, or crosses the middle. The crossing case is the best suffix of the left half
plus the best prefix of the right half, which a linear scan finds.

**Algorithm.** Recursively solve `[lo, mid]` and `[mid + 1, hi]`; compute the best sum ending at
`mid` (scanning left) and the best sum starting at `mid + 1` (scanning right); return the largest
of the three candidates.

```python
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        def solve(lo, hi):
            if lo == hi:
                return nums[lo]
            mid = (lo + hi) // 2
            best_left = cur = nums[mid]
            for i in range(mid - 1, lo - 1, -1):
                cur += nums[i]
                best_left = max(best_left, cur)
            best_right = cur = nums[mid + 1]
            for i in range(mid + 2, hi + 1):
                cur += nums[i]
                best_right = max(best_right, cur)
            return max(solve(lo, mid), solve(mid + 1, hi), best_left + best_right)

        return solve(0, len(nums) - 1)
```

- **Time:** `O(n log n)` — linear work per level, `log n` levels.
- **Space:** `O(log n)` recursion depth.

(Returning a prefix/suffix/total summary from each half brings this to `O(n)`, but the next
approach is simpler.)

## Approach 3: Kadane's Algorithm (Optimal)

**Intuition.** Let `cur` be the best sum of a subarray that ends at the current index. For the
next element `x`, either we extend that subarray (`cur + x`) or start fresh at `x`; starting
fresh wins exactly when `cur` is negative. The answer is the largest `cur` ever seen.

**Algorithm.**

1. `cur = best = nums[0]`.
2. For each subsequent `x`: `cur = max(x, cur + x)`, then `best = max(best, cur)`.
3. Return `best`.

Initialising with `nums[0]` rather than `0` matters: when every element is negative the answer is
the single largest (least negative) element, not `0`.

```python
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        cur = best = nums[0]
        for x in nums[1:]:
            cur = x if cur < 0 else cur + x
            if cur > best:
                best = cur
        return best
```

- **Time:** `O(n)`.
- **Space:** `O(1)` extra (the slice `nums[1:]` can be replaced by an index loop if needed).
