You are given an array `intervals` with `intervals[i] = [start_i, end_i]`. Find the **smallest number of intervals** you must delete so that no two of the remaining intervals overlap.

Here two intervals overlap only if they share more than a single point: `[1,2]` and `[2,3]` merely touch and are allowed to coexist, while `[1,3]` and `[2,4]` overlap.

<!-- examples -->

**Constraints:**

- `1 <= intervals.length <= 10^5`
- `intervals[i].length == 2`
- `-5 * 10^4 <= start_i < end_i <= 5 * 10^4`
