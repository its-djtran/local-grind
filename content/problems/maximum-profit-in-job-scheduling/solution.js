/**
 * @param {number[]} startTime
 * @param {number[]} endTime
 * @param {number[]} profit
 * @return {number}
 */
var jobScheduling = function(startTime, endTime, profit) {
    const n = startTime.length;
    const order = Array.from({ length: n }, (_, i) => i);
    order.sort((a, b) => endTime[a] - endTime[b]);
    const ends = order.map(i => endTime[i]);
    const dp = new Array(n + 1).fill(0);
    for (let i = 1; i <= n; i++) {
        const job = order[i - 1];
        const s = startTime[job];
        // j = number of jobs with end <= s (bisect_right)
        let lo = 0, hi = i - 1;
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (ends[mid] <= s) lo = mid + 1;
            else hi = mid;
        }
        dp[i] = Math.max(dp[i - 1], dp[lo] + profit[job]);
    }
    return dp[n];
};
