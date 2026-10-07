/**
 * @param {number[]} heights
 * @return {number}
 */
var largestRectangleArea = function(heights) {
    const n = heights.length;
    const stack = new Int32Array(n + 1);
    let top = 0, best = 0;
    for (let i = 0; i <= n; i++) {
        const h = i < n ? heights[i] : 0;
        while (top > 0 && heights[stack[top - 1]] >= h) {
            const height = heights[stack[--top]];
            const left = top > 0 ? stack[top - 1] : -1;
            best = Math.max(best, height * (i - left - 1));
        }
        stack[top++] = i;
    }
    return best;
};
