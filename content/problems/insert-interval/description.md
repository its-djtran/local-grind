You are given a list `intervals` where `intervals[i] = [start_i, end_i]`. The intervals do not overlap one another and are sorted in increasing order of `start_i`. You are also given one more interval, `newInterval = [start, end]`.

Add `newInterval` to the list so that the result is **still sorted by start and still free of overlaps**. Wherever `newInterval` overlaps existing intervals, combine them into a single interval. Two intervals that share an endpoint (such as `[1,2]` and `[2,3]`) count as overlapping.

Return the resulting list. You may build a new list rather than modifying `intervals` in place.

<!-- examples -->

**Constraints:**

- `0 <= intervals.length <= 10^4`
- `intervals[i].length == 2`
- `0 <= start_i <= end_i <= 10^5`
- `intervals` is sorted by `start_i` in ascending order, and no two of its intervals overlap.
- `newInterval.length == 2`
- `0 <= start <= end <= 10^5`
