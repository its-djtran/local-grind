An array of **distinct** integers was sorted in increasing order and then possibly **rotated** around some pivot index `k` (`0 <= k < nums.length`), so that it reads `[nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]` in terms of the original sorted order. For instance, `[0,1,2,4,5,6,7]` rotated at index `3` becomes `[4,5,6,7,0,1,2]`.

Given the rotated array `nums` and an integer `target`, return the index of `target` in `nums`, or `-1` if it does not occur.

Your algorithm must run in `O(log n)` time.

<!-- examples -->

**Constraints:**

- `1 <= nums.length <= 5000`
- `-10^4 <= nums[i] <= 10^4`
- All values in `nums` are distinct.
- `nums` is an ascending array that may have been rotated.
- `-10^4 <= target <= 10^4`
