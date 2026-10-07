You are given a list of meetings, where `intervals[i] = [start_i, end_i]` means a meeting begins at time `start_i` and finishes at time `end_i`. Decide whether one person could sit through **every** meeting, which is possible only if no two meetings overlap in time.

A meeting that ends at time `t` does not conflict with a meeting that begins at time `t`; the person can go straight from one to the next.

Return `true` if all meetings can be attended and `false` otherwise.

<!-- examples -->

**Constraints:**

- `0 <= intervals.length <= 10^4`
- `intervals[i].length == 2`
- `0 <= start_i < end_i <= 10^6`
