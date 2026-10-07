/**
 * @param {number[][]} matrix
 * @return {void}
 */
var setZeroes = function(matrix) {
    const m = matrix.length, n = matrix[0].length;
    const top = matrix[0];
    const firstRow = top.some(v => v === 0);
    let firstCol = false;
    for (let i = 0; i < m; i++) if (matrix[i][0] === 0) firstCol = true;
    for (let i = 1; i < m; i++) {
        const row = matrix[i];
        for (let j = 1; j < n; j++) {
            if (row[j] === 0) {
                row[0] = 0;
                top[j] = 0;
            }
        }
    }
    for (let i = 1; i < m; i++) {
        const row = matrix[i];
        if (row[0] === 0) {
            row.fill(0, 1);
        } else {
            for (let j = 1; j < n; j++) if (top[j] === 0) row[j] = 0;
        }
    }
    if (firstRow) top.fill(0);
    if (firstCol) for (let i = 0; i < m; i++) matrix[i][0] = 0;
};
