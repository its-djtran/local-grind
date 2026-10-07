/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function(height) {
    let lo = 0, hi = height.length - 1;
    let leftMax = 0, rightMax = 0, total = 0;
    while (lo <= hi) {
        leftMax = Math.max(leftMax, height[lo]);
        rightMax = Math.max(rightMax, height[hi]);
        if (leftMax <= rightMax) {
            total += leftMax - height[lo++];
        } else {
            total += rightMax - height[hi--];
        }
    }
    return total;
};
