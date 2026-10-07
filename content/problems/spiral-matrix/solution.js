/**
 * @param {number[][]} matrix
 * @return {number[]}
 */
var spiralOrder = function(matrix) {
    let top = 0, bottom = matrix.length - 1;
    let left = 0, right = matrix[0].length - 1;
    const out = [];
    while (top <= bottom && left <= right) {
        for (let j = left; j <= right; j++) out.push(matrix[top][j]);
        for (let i = top + 1; i <= bottom; i++) out.push(matrix[i][right]);
        if (top < bottom) for (let j = right - 1; j >= left; j--) out.push(matrix[bottom][j]);
        if (left < right) for (let i = bottom - 1; i > top; i--) out.push(matrix[i][left]);
        top++; bottom--; left++; right--;
    }
    return out;
};
