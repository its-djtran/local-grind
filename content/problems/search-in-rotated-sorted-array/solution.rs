impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Half-open window [lo, hi) keeps the indices unsigned without underflow.
        let (mut lo, mut hi) = (0usize, nums.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[lo] <= nums[mid] {
                if nums[lo] <= target && target < nums[mid] {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            } else if nums[mid] < target && target <= nums[hi - 1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        -1
    }
}
