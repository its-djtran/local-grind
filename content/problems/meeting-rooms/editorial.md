## Approach 1: Compare Every Pair

**Intuition.** Two meetings `[s1, e1]` and `[s2, e2]` clash exactly when each starts before the
other ends: `s1 < e2 and s2 < e1`. Strict inequalities let back-to-back meetings pass. Test all
pairs.

```python
class Solution:
    def canAttendMeetings(self, intervals: List[List[int]]) -> bool:
        n = len(intervals)
        for i in range(n):
            for j in range(i + 1, n):
                if intervals[i][0] < intervals[j][1] and intervals[j][0] < intervals[i][1]:
                    return False
        return True
```

- **Time:** `O(n^2)`.
- **Space:** `O(1)`.

## Approach 2: Sort by Start Time (Optimal)

**Intuition.** Lay the meetings out in order of start time. If any two meetings overlap, then in
particular some meeting starts before the meeting immediately preceding it has ended: the
earlier-starting meeting of an overlapping pair "covers" everything up to the later one's start,
so the overlap shows up between neighbours. Conversely, a neighbour clash is an overlap.

**Algorithm.**

1. Sort `intervals` by start.
2. For each consecutive pair, if `intervals[i][0] < intervals[i - 1][1]`, return `false`.
3. Return `true` (this also covers zero or one meeting).

```python
class Solution:
    def canAttendMeetings(self, intervals: List[List[int]]) -> bool:
        intervals.sort()
        for i in range(1, len(intervals)):
            if intervals[i][0] < intervals[i - 1][1]:
                return False
        return True
```

- **Time:** `O(n log n)` for the sort.
- **Space:** `O(1)` beyond the sort.
