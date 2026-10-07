/**
 * @param {number[][]} heights
 * @return {number[][]}
 */
var pacificAtlantic = function(heights) {
    const m = heights.length, n = heights[0].length;
    const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    const climb = (starts) => {
        const seen = Array.from({ length: m }, () => new Array(n).fill(false));
        const queue = [];
        for (const [r, c] of starts) {
            if (!seen[r][c]) {
                seen[r][c] = true;
                queue.push([r, c]);
            }
        }
        for (let head = 0; head < queue.length; head++) {
            const [r, c] = queue[head];
            const h = heights[r][c];
            for (const [dr, dc] of dirs) {
                const nr = r + dr, nc = c + dc;
                if (nr >= 0 && nr < m && nc >= 0 && nc < n && !seen[nr][nc] && heights[nr][nc] >= h) {
                    seen[nr][nc] = true;
                    queue.push([nr, nc]);
                }
            }
        }
        return seen;
    };

    const pacStarts = [], atlStarts = [];
    for (let c = 0; c < n; c++) {
        pacStarts.push([0, c]);
        atlStarts.push([m - 1, c]);
    }
    for (let r = 0; r < m; r++) {
        pacStarts.push([r, 0]);
        atlStarts.push([r, n - 1]);
    }
    const pacific = climb(pacStarts);
    const atlantic = climb(atlStarts);
    const res = [];
    for (let r = 0; r < m; r++)
        for (let c = 0; c < n; c++)
            if (pacific[r][c] && atlantic[r][c]) res.push([r, c]);
    return res;
};
