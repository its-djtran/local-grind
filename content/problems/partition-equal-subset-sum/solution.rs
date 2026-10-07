impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false;
        }
        let half = (total / 2) as usize;
        let mut dp = vec![false; half + 1];
        dp[0] = true;
        for x in nums {
            let x = x as usize;
            for s in (x..=half).rev() {
                dp[s] = dp[s] || dp[s - x];
            }
            if dp[half] {
                return true;
            }
        }
        dp[half]
    }
}
