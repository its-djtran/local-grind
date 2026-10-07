An array of `n` **distinct** integers was sorted in increasing order and then **rotated** some number of times, between `1` and `n`. Rotating once moves the last element to the front: `[a0, a1, ..., a(n-1)]` becomes `[a(n-1), a0, a1, ..., a(n-2)]`. For example, `[0,1,2,4,5,6,7]` becomes `[4,5,6,7,0,1,2]` after 4 rotations, and is unchanged after 7.

Given the rotated array `nums`, return its **smallest** element.

Your algorithm must run in `O(log n)` time.

<!-- examples -->

**Constraints:**

- `n == nums.length`
- `1 <= n <= 5000`
- `-5000 <= nums[i] <= 5000`
- All values in `nums` are distinct.
- `nums` is a sorted array that was rotated between `1` and `n` times.
