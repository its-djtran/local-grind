/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canJump = function(nums) {
    let far = 0;
    const last = nums.length - 1;
    for (let i = 0; i < nums.length; i++) {
        if (i > far) return false;
        far = Math.max(far, i + nums[i]);
        if (far >= last) return true;
    }
    return true;
};
