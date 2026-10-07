## Approach 1: Repeated Pairwise Merging

**Intuition.** Ignore the fact that the input is sorted: drop `newInterval` into the list and keep
merging any two overlapping intervals until no overlapping pair remains. Then sort.

**Algorithm.**

1. Append `newInterval` to a copy of `intervals`.
2. Repeatedly look for a pair `a`, `b` with `a[0] <= b[1]` and `b[0] <= a[1]`; replace them with
   `[min(a[0], b[0]), max(a[1], b[1])]`. Stop when a full scan finds no pair.
3. Sort the survivors by start.

```python
class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        items = [list(iv) for iv in intervals] + [list(newInterval)]
        changed = True
        while changed:
            changed = False
            for i in range(len(items)):
                for j in range(i + 1, len(items)):
                    a, b = items[i], items[j]
                    if a[0] <= b[1] and b[0] <= a[1]:
                        items[i] = [min(a[0], b[0]), max(a[1], b[1])]
                        items.pop(j)
                        changed = True
                        break
                if changed:
                    break
        return sorted(items)
```

- **Time:** `O(n^3)` in the worst case; even a single scan with nothing to merge costs `O(n^2)`.
- **Space:** `O(n)`.

## Approach 2: Append, Sort, Merge

**Intuition.** This is the general "merge intervals" routine: add the new interval, sort by start,
and sweep once, extending the last output interval whenever the next one overlaps it.

```python
class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        merged = []
        for s, e in sorted(intervals + [newInterval]):
            if merged and s <= merged[-1][1]:
                merged[-1][1] = max(merged[-1][1], e)
            else:
                merged.append([s, e])
        return merged
```

- **Time:** `O(n log n)` for the sort, which the sorted input makes unnecessary.
- **Space:** `O(n)`.

## Approach 3: Single Linear Pass (Optimal)

**Intuition.** Because the input is sorted and disjoint, the intervals fall into three consecutive
groups relative to `newInterval = [s, e]`:

- those that end strictly before `s` (entirely to the left),
- those that overlap `[s, e]`,
- those that start strictly after `e` (entirely to the right).

The left and right groups are copied unchanged. The middle group is absorbed into `newInterval`
by widening it to the smallest start and the largest end seen.

**Algorithm.**

1. Copy intervals while `intervals[i][1] < s`.
2. While `intervals[i][0] <= e`, set `s = min(s, intervals[i][0])` and
   `e = max(e, intervals[i][1])`.
3. Emit `[s, e]`, then copy the remaining intervals.

The comparisons use `<` in step 1 and `<=` in step 2 so that intervals sharing an endpoint with
`newInterval` are merged.

```python
class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        s, e = newInterval
        n, i = len(intervals), 0
        result = []
        while i < n and intervals[i][1] < s:
            result.append(intervals[i])
            i += 1
        while i < n and intervals[i][0] <= e:
            s = min(s, intervals[i][0])
            e = max(e, intervals[i][1])
            i += 1
        result.append([s, e])
        result.extend(intervals[i:])
        return result
```

- **Time:** `O(n)` — each interval is looked at once.
- **Space:** `O(n)` for the output (`O(1)` extra).
