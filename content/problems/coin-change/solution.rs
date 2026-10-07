impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let inf = amount as i32 + 1;
        let mut dp = vec![inf; amount + 1];
        dp[0] = 0;
        for c in coins {
            let c = c as usize;
            for x in c..=amount {
                dp[x] = dp[x].min(dp[x - c] + 1);
            }
        }
        if dp[amount] == inf { -1 } else { dp[amount] }
    }
}
