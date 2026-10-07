Given an integer `n`, build and return an array `ans` with `n + 1` entries where `ans[i]` is the number of `1` bits in the binary representation of `i`, for every `0 <= i <= n`.

<!-- examples -->

**Constraints:**

- `0 <= n <= 10^5`

**Follow-up:**

- An `O(n log n)` solution that counts each number separately is straightforward. Can you produce the whole array in linear time with a single pass?
- Can you do it without a built-in population-count function (such as `int.bit_count` or `__builtin_popcount`)?
