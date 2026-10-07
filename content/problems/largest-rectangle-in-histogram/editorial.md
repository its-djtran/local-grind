## Approach 1: Brute Force Over Ranges

**Intuition.** A rectangle spanning bars `i..j` can be as tall as the shortest bar in that range.
Fix the left end `i`, extend `j` to the right while maintaining the running minimum, and score
each range.

```python
class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        best = 0
        for i in range(len(heights)):
            low = heights[i]
            for j in range(i, len(heights)):
                low = min(low, heights[j])
                best = max(best, low * (j - i + 1))
        return best
```

- **Time:** `O(n^2)`.
- **Space:** `O(1)`.

## Approach 2: Divide and Conquer

**Intuition.** Within a range, the shortest bar either belongs to the best rectangle, which then
spans the whole range at that height, or it does not, in which case the best rectangle lies
entirely to its left or entirely to its right. Recurse on both sides.

```python
class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        def solve(lo: int, hi: int) -> int:
            if lo > hi:
                return 0
            m = min(range(lo, hi + 1), key=heights.__getitem__)
            return max(heights[m] * (hi - lo + 1), solve(lo, m - 1), solve(m + 1, hi))

        return solve(0, len(heights) - 1)
```

- **Time:** `O(n log n)` on balanced splits, but `O(n^2)` on sorted input, where the minimum is
  always at one end. (A segment tree for the range minimum brings the worst case to `O(n log n)`.)
- **Space:** `O(n)` recursion depth in the worst case.

## Approach 3: Monotonic Stack (Optimal)

**Intuition.** For each bar, the widest rectangle that uses its full height stretches to the
nearest strictly shorter bar on each side. Scan left to right with a stack of indices whose
heights are increasing. When bar `i` is shorter than the bar on top of the stack, that top bar
cannot extend past `i`, so `i` is its right boundary; and the index just below it on the stack is
the nearest shorter bar on its left. Pop it, compute its rectangle, and repeat while the top is
still taller than `heights[i]`.

**Algorithm.**

1. Iterate over the heights with a trailing `0` appended, so every bar is eventually popped.
2. For each index `i` with height `h`: while the stack is non-empty and `heights[top] >= h`, pop
   `top`; its rectangle has height `heights[top]` and width `i - left - 1`, where `left` is the new
   top of the stack (or `-1` when the stack is empty). Update the best area.
3. Push `i`.

Popping on equal heights is fine: the last bar of a run of equal heights computes the full width
for the whole run.

```python
class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        stack, best = [], 0
        for i, h in enumerate(heights + [0]):
            while stack and heights[stack[-1]] >= h:
                height = heights[stack.pop()]
                left = stack[-1] if stack else -1
                best = max(best, height * (i - left - 1))
            stack.append(i)
        return best
```

- **Time:** `O(n)` — each index is pushed and popped once.
- **Space:** `O(n)` for the stack.
