You are given a list of meetings where `intervals[i] = [start_i, end_i]` means a meeting occupies a room from time `start_i` up to time `end_i`. Find the **minimum number of conference rooms** that lets every meeting take place without two meetings sharing a room at the same time.

A room vacated at time `t` can immediately host a meeting that begins at time `t`.

<!-- examples -->

**Constraints:**

- `1 <= intervals.length <= 10^4`
- `intervals[i].length == 2`
- `0 <= start_i < end_i <= 10^6`
