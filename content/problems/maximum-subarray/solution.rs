impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cur = nums[0];
        let mut best = nums[0];
        for &x in &nums[1..] {
            cur = if cur < 0 { x } else { cur + x };
            best = best.max(cur);
        }
        best
    }
}
