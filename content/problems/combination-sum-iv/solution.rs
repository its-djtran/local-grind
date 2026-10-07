impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let target = target as usize;
        // Intermediate counts can overflow; wrap mod 2^32 since only the answer fits in i32.
        let mut dp = vec![0u32; target + 1];
        dp[0] = 1;
        for t in 1..=target {
            let mut total = 0u32;
            for &x in &nums {
                let x = x as usize;
                if x > t {
                    break;
                }
                total = total.wrapping_add(dp[t - x]);
            }
            dp[t] = total;
        }
        dp[target] as i32
    }
}
