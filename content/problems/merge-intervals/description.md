You are given an array `intervals` where `intervals[i] = [start_i, end_i]` describes a closed range on the number line. The array is in no particular order.

Combine every group of overlapping intervals into one interval that spans the whole group, and return the resulting list of intervals, which must not overlap each other and must together cover exactly the same points as the input. Intervals that only share an endpoint, like `[1,4]` and `[4,5]`, are considered overlapping.

The merged intervals may be returned in any order.

<!-- examples -->

**Constraints:**

- `1 <= intervals.length <= 10^4`
- `intervals[i].length == 2`
- `0 <= start_i <= end_i <= 10^4`
