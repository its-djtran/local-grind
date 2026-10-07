impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let total = (m + n - 2) as i64;
        let k = (m.min(n) - 1) as i64;
        let mut result: i64 = 1;
        for i in 1..=k {
            result = result * (total - k + i) / i;
        }
        result as i32
    }
}
