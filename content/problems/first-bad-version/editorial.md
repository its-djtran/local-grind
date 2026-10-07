## Approach 1: Linear Scan

**Intuition.** Test versions in order and return the first one the API reports as bad. One is
guaranteed to exist, so the loop always returns.

```python
class Solution:
    def firstBadVersion(self, n: int) -> int:
        for version in range(1, n + 1):
            if isBadVersion(version):
                return version
```

- **Time:** `O(n)` API calls — over two billion when `n = 2^31 - 1` and the defect is recent.
- **Space:** `O(1)`.

## Approach 2: Binary Search (Optimal)

**Intuition.** Listing `isBadVersion(1), ..., isBadVersion(n)` gives a run of `false` followed by a
run of `true`. We need the boundary between the runs, and a single query at any position tells us
which side of the boundary it is on:

- If `mid` is bad, the first bad version is `mid` itself or something before it.
- If `mid` is good, the first bad version is strictly after `mid`.

Either way, about half the candidates are eliminated per query.

**Algorithm.**

1. Keep the candidate range `[lo, hi] = [1, n]`; the answer is always inside it.
2. While `lo < hi`:
   - `mid = (lo + hi) // 2`.
   - If `isBadVersion(mid)`, set `hi = mid` (keep `mid`, since it might be the answer).
   - Otherwise set `lo = mid + 1`.
3. When `lo == hi`, the range holds a single version: return it.

Because `mid < hi` whenever `lo < hi`, both updates strictly shrink the range, so the loop ends.
In languages with 32-bit integers, `lo + hi` can overflow when `n` is near `2^31 - 1`; write
`mid = lo + (hi - lo) // 2` there. Python integers are unbounded, so either form works.

```python
class Solution:
    def firstBadVersion(self, n: int) -> int:
        lo, hi = 1, n
        while lo < hi:
            mid = (lo + hi) // 2
            if isBadVersion(mid):
                hi = mid
            else:
                lo = mid + 1
        return lo
```

- **Time:** `O(log n)` API calls — at most 31 for any valid `n`.
- **Space:** `O(1)`.
