impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low = prices[0];
        let mut best = 0;
        for &p in &prices {
            if p < low {
                low = p;
            } else {
                best = best.max(p - low);
            }
        }
        best
    }
}
