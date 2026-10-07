/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
var uniquePaths = function(m, n) {
    const total = m + n - 2, k = Math.min(m, n) - 1;
    let result = 1;
    for (let i = 1; i <= k; i++) {
        result = result * (total - k + i) / i;
    }
    return result;
};
