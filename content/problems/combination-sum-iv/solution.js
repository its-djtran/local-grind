/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var combinationSum4 = function(nums, target) {
    nums = [...nums].sort((a, b) => a - b);
    const dp = new Array(target + 1).fill(0);
    dp[0] = 1;
    for (let t = 1; t <= target; t++) {
        let total = 0;
        for (const x of nums) {
            if (x > t) break;
            // Intermediate counts can exceed 2^53; only the answer fits in 32 bits,
            // so count modulo 2^32 (both addends < 2^32, so the sum is exact).
            total = (total + dp[t - x]) >>> 0;
        }
        dp[t] = total;
    }
    return dp[target];
};
