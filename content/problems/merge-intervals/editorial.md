## Approach 1: Merge Into a Growing List

**Intuition.** Process the intervals one at a time, keeping a list of disjoint merged intervals.
A new interval may overlap several of them at once, so pull out every overlapping one, union
them all with the new interval, and put the combined interval back.

**Algorithm.**

1. Start with an empty list `merged`.
2. For each interval `[s, e]`, split `merged` into intervals that overlap `[s, e]`
   (`a <= e and s <= b`) and those that do not. Widen `[s, e]` to cover all the overlapping ones
   and store it together with the non-overlapping ones.

```python
class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        merged = []
        for s, e in intervals:
            keep = []
            for a, b in merged:
                if a <= e and s <= b:
                    s, e = min(s, a), max(e, b)
                else:
                    keep.append([a, b])
            keep.append([s, e])
            merged = keep
        return merged
```

- **Time:** `O(n^2)` — when few intervals overlap, every new interval scans a list of size
  `O(n)`.
- **Space:** `O(n)`.

## Approach 2: Sort, Then Sweep (Optimal)

**Intuition.** Once the intervals are sorted by start, any interval that overlaps an earlier group
must overlap the *last* group built so far: every earlier group ends before the last group
begins. So a single left-to-right sweep suffices.

**Algorithm.**

1. Sort `intervals` by start.
2. For each `[s, e]`: if `merged` is non-empty and `s <= merged[-1][1]`, set
   `merged[-1][1] = max(merged[-1][1], e)`; otherwise append `[s, e]`.

Taking the `max` matters for nested intervals such as `[1,10]` followed by `[2,3]`; simply
overwriting the end would shrink the group.

```python
class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals.sort(key=lambda iv: iv[0])
        merged = []
        for s, e in intervals:
            if merged and s <= merged[-1][1]:
                if e > merged[-1][1]:
                    merged[-1][1] = e
            else:
                merged.append([s, e])
        return merged
```

- **Time:** `O(n log n)` for the sort; the sweep is linear.
- **Space:** `O(n)` for the output (plus the sort's own memory).
