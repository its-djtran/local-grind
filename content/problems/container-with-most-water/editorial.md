## Approach 1: Brute Force

Evaluate the container for every pair of lines.

```python
class Solution:
    def maxArea(self, height: List[int]) -> int:
        best = 0
        n = len(height)
        for i in range(n):
            for j in range(i + 1, n):
                best = max(best, (j - i) * min(height[i], height[j]))
        return best
```

- **Time:** `O(n^2)` — about `5 * 10^9` pairs at the maximum size.
- **Space:** `O(1)`.

## Approach 2: Two Pointers From Both Ends (Optimal)

**Intuition.** Begin with the widest possible container, `i = 0` and `j = n - 1`. Suppose
`height[i] <= height[j]`. Any other container that keeps line `i` and uses some `j' < j` is
narrower and its water level is still capped by `height[i]`, so it cannot beat the current one.
Line `i` therefore has nothing left to offer and can be discarded by moving `i` one step right.
The symmetric argument applies when the right line is shorter. Each step discards one line, and
the best container is never discarded before it is measured.

**Algorithm.**

1. `i, j = 0, n - 1`, `best = 0`.
2. While `i < j`: update `best` with `(j - i) * min(height[i], height[j])`, then move the
   pointer at the shorter line inward (either one when they are equal).
3. Return `best`.

```python
class Solution:
    def maxArea(self, height: List[int]) -> int:
        i, j = 0, len(height) - 1
        best = 0
        while i < j:
            hi, hj = height[i], height[j]
            if hi < hj:
                area = (j - i) * hi
                i += 1
            else:
                area = (j - i) * hj
                j -= 1
            if area > best:
                best = area
        return best
```

- **Time:** `O(n)` — the pointers meet after `n - 1` moves.
- **Space:** `O(1)`.
