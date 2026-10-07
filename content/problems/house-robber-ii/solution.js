/**
 * @param {number[]} nums
 * @return {number}
 */
var rob = function(nums) {
    const n = nums.length;
    if (n === 1) return nums[0];
    const line = (lo, hi) => {
        let prev = 0, cur = 0;
        for (let i = lo; i < hi; i++) {
            [prev, cur] = [cur, Math.max(cur, prev + nums[i])];
        }
        return cur;
    };
    return Math.max(line(0, n - 1), line(1, n));
};
