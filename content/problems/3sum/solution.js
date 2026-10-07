/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function(nums) {
    nums.sort((a, b) => a - b);
    const n = nums.length;
    const res = [];
    for (let i = 0; i < n - 2; i++) {
        const a = nums[i];
        if (a > 0) break;
        if (i > 0 && a === nums[i - 1]) continue;
        let lo = i + 1, hi = n - 1;
        while (lo < hi) {
            const s = a + nums[lo] + nums[hi];
            if (s < 0) lo++;
            else if (s > 0) hi--;
            else {
                res.push([a, nums[lo], nums[hi]]);
                lo++;
                hi--;
                while (lo < hi && nums[lo] === nums[lo - 1]) lo++;
            }
        }
    }
    return res;
};
