impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut hi, mut lo, mut best) = (nums[0], nums[0], nums[0]);
        for &x in &nums[1..] {
            let (a, b) = (hi * x, lo * x);
            hi = x.max(a).max(b);
            lo = x.min(a).min(b);
            best = best.max(hi);
        }
        best
    }
}
