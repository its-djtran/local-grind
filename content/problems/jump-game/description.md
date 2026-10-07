You start on the first element of an integer array `nums`. The value `nums[i]` is the **longest jump** you may make from index `i`: from there you can move forward to any index from `i + 1` up to `i + nums[i]` (a value of `0` means you cannot move).

Return `true` if some sequence of jumps takes you to the **last index**, and `false` otherwise.

<!-- examples -->

**Constraints:**

- `1 <= nums.length <= 10^4`
- `0 <= nums[i] <= 10^5`
