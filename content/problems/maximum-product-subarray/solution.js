/**
 * @param {number[]} nums
 * @return {number}
 */
var maxProduct = function(nums) {
    let hi = nums[0], lo = nums[0], best = nums[0];
    for (let i = 1; i < nums.length; i++) {
        const x = nums[i];
        const a = hi * x, b = lo * x;
        hi = Math.max(x, a, b);
        lo = Math.min(x, a, b);
        if (hi > best) best = hi;
    }
    return best;
};
