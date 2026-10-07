## Approach 1: Brute Force

**Intuition.** Water above column `i` can rise to `min(tallest bar on the left, tallest bar on the
right)`, where both maxima include column `i` itself; whatever is above the bar up to that level is
water. Compute both maxima from scratch for every column.

```python
class Solution:
    def trap(self, height: List[int]) -> int:
        total = 0
        for i in range(len(height)):
            level = min(max(height[:i + 1]), max(height[i:]))
            total += level - height[i]
        return total
```

- **Time:** `O(n^2)` — each column scans the whole array.
- **Space:** `O(n)` for the slices (`O(1)` with explicit loops).

## Approach 2: Prefix and Suffix Maxima

**Intuition.** The left maximum for column `i` is the left maximum for `i - 1` combined with
`height[i]`, and symmetrically for the right. Two linear sweeps precompute both arrays.

```python
class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        left, right = [0] * n, [0] * n
        best = 0
        for i in range(n):
            best = max(best, height[i])
            left[i] = best
        best = 0
        for i in range(n - 1, -1, -1):
            best = max(best, height[i])
            right[i] = best
        return sum(min(left[i], right[i]) - height[i] for i in range(n))
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the two arrays.

## Approach 3: Monotonic Stack

**Intuition.** Instead of column by column, fill water layer by layer. Keep a stack of indices
with non-increasing heights. When a taller bar arrives at `i`, the bar on top of the stack is the
floor of a basin whose walls are the new top of the stack and `i`. That basin holds
`(min(wall heights) - floor height) * width` units, where the width is the distance between the
walls minus one.

```python
class Solution:
    def trap(self, height: List[int]) -> int:
        stack, total = [], 0
        for i, h in enumerate(height):
            while stack and height[stack[-1]] < h:
                floor = height[stack.pop()]
                if not stack:
                    break
                left = stack[-1]
                total += (min(height[left], h) - floor) * (i - left - 1)
            stack.append(i)
        return total
```

- **Time:** `O(n)` — every index is pushed and popped at most once.
- **Space:** `O(n)` for the stack.

## Approach 4: Two Pointers (Optimal)

**Intuition.** Put pointers `lo` and `hi` at both ends and track `left_max` (tallest bar in
`height[0..lo]`) and `right_max` (tallest in `height[hi..n-1]`). If `left_max <= right_max`, then
the true right maximum for column `lo` is at least `right_max`, so the water level at `lo` is
exactly `left_max`: it can be settled now and `lo` advanced. Otherwise the same argument settles
`hi`. Each step finishes one column without any extra arrays.

**Algorithm.**

1. `lo = 0`, `hi = n - 1`, `left_max = right_max = total = 0`.
2. While `lo <= hi`:
   - update `left_max` with `height[lo]` and `right_max` with `height[hi]`;
   - if `left_max <= right_max`, add `left_max - height[lo]` and increment `lo`;
   - otherwise add `right_max - height[hi]` and decrement `hi`.
3. Return `total`.

```python
class Solution:
    def trap(self, height: List[int]) -> int:
        lo, hi = 0, len(height) - 1
        left_max = right_max = total = 0
        while lo <= hi:
            left_max = max(left_max, height[lo])
            right_max = max(right_max, height[hi])
            if left_max <= right_max:
                total += left_max - height[lo]
                lo += 1
            else:
                total += right_max - height[hi]
                hi -= 1
        return total
```

- **Time:** `O(n)`.
- **Space:** `O(1)`.
