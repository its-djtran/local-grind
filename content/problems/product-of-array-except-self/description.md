You are given an integer array `nums`. Build and return an array `answer` of the same length, where `answer[i]` equals the product of **all elements of `nums` other than `nums[i]`**.

It is guaranteed that the product of any prefix or suffix of `nums` fits in a 32-bit signed integer.

Your algorithm must run in `O(n)` time and must **not** use division.

<!-- examples -->

**Constraints:**

- `2 <= nums.length <= 10^5`
- `-30 <= nums[i] <= 30`
- The product of any prefix or suffix of `nums` fits in a 32-bit integer.

**Follow-up:** Can you do it with `O(1)` extra space? The returned `answer` array does not count as extra space.
