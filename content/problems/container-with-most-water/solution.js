/**
 * @param {number[]} height
 * @return {number}
 */
var maxArea = function(height) {
    let i = 0, j = height.length - 1, best = 0;
    while (i < j) {
        let area;
        if (height[i] < height[j]) {
            area = (j - i) * height[i];
            i++;
        } else {
            area = (j - i) * height[j];
            j--;
        }
        if (area > best) best = area;
    }
    return best;
};
