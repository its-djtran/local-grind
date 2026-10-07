/**
 * @param {number[][]} mat
 * @return {number[][]}
 */
var updateMatrix = function(mat) {
    const m = mat.length, n = mat[0].length;
    const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    const dist = Array.from({ length: m }, () => new Array(n).fill(-1));
    const queue = [];
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            if (mat[i][j] === 0) {
                dist[i][j] = 0;
                queue.push([i, j]);
            }
        }
    }
    for (let head = 0; head < queue.length; head++) {
        const [r, c] = queue[head];
        const d = dist[r][c] + 1;
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < m && nc >= 0 && nc < n && dist[nr][nc] === -1) {
                dist[nr][nc] = d;
                queue.push([nr, nc]);
            }
        }
    }
    return dist;
};
