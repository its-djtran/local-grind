/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function(n) {
    let prev = 1, cur = 1;
    for (let i = 1; i < n; i++) {
        [prev, cur] = [cur, prev + cur];
    }
    return cur;
};
