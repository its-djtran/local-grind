Removing the fewest intervals is equivalent to keeping the largest set of pairwise
non-overlapping intervals; the answer is `n` minus the size of that set.

## Approach 1: Dynamic Programming Over Sorted Intervals

**Intuition.** Sort by start. Let `keep[i]` be the largest compatible set that ends with interval
`i`. Interval `i` can follow interval `j < i` whenever `j` ends no later than `i` starts.

**Algorithm.**

1. Sort the intervals by start.
2. `keep[i] = 1 + max(keep[j])` over all `j < i` with `end_j <= start_i` (or `1` if none).
3. Return `n - max(keep)`.

```python
class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        intervals.sort()
        n = len(intervals)
        keep = [1] * n
        for i in range(n):
            for j in range(i):
                if intervals[j][1] <= intervals[i][0]:
                    keep[i] = max(keep[i], keep[j] + 1)
        return n - max(keep)
```

- **Time:** `O(n^2)` — far too slow for `10^5` intervals.
- **Space:** `O(n)`.

## Approach 2: Greedy by Earliest End (Optimal)

**Intuition.** Think about which interval to keep first. Choosing the one that **ends earliest**
can never hurt: any optimal selection can swap its first interval for the earliest-ending one
without creating a conflict, because that one ends no later. After committing to it, discard
everything that overlaps it and repeat on what is left. This is the classic activity-selection
argument.

**Algorithm.**

1. Sort intervals by end.
2. Track `last_end`, the end of the most recently kept interval (start at negative infinity).
3. For each `[s, e]` in order: if `s >= last_end`, keep it and set `last_end = e`; otherwise it
   overlaps something already kept, so count it as removed.

The test is `s >= last_end` (not `>`) because touching intervals do not overlap.

```python
class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        intervals.sort(key=lambda iv: iv[1])
        removed = 0
        last_end = float("-inf")
        for s, e in intervals:
            if s >= last_end:
                last_end = e
            else:
                removed += 1
        return removed
```

- **Time:** `O(n log n)` for the sort, then one linear pass.
- **Space:** `O(1)` beyond the sort.
