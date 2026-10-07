/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canPartition = function(nums) {
    let total = 0;
    for (const x of nums) total += x;
    if (total % 2) return false;
    const half = total / 2;
    const dp = new Uint8Array(half + 1);
    dp[0] = 1;
    for (const x of nums) {
        for (let s = half; s >= x; s--) {
            if (dp[s - x]) dp[s] = 1;
        }
        if (dp[half]) return true;
    }
    return dp[half] === 1;
};
