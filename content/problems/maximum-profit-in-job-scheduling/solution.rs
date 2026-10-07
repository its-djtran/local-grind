impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut jobs: Vec<(i32, i32, i32)> = (0..n)
            .map(|i| (end_time[i], start_time[i], profit[i]))
            .collect();
        jobs.sort_unstable();
        let ends: Vec<i32> = jobs.iter().map(|j| j.0).collect();
        let mut dp = vec![0; n + 1];
        for (i, &(_, s, p)) in jobs.iter().enumerate() {
            let j = ends[..i].partition_point(|&e| e <= s);
            dp[i + 1] = dp[i].max(dp[j] + p);
        }
        dp[n]
    }
}
