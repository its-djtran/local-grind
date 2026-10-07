/**
 * @param {string[][]} grid
 * @return {number}
 */
var numIslands = function(grid) {
    const m = grid.length, n = grid[0].length;
    const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    let count = 0;
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            if (grid[i][j] !== "1") continue;
            count++;
            grid[i][j] = "0";
            const stack = [[i, j]];
            while (stack.length) {
                const [r, c] = stack.pop();
                for (const [dr, dc] of dirs) {
                    const nr = r + dr, nc = c + dc;
                    if (nr >= 0 && nr < m && nc >= 0 && nc < n && grid[nr][nc] === "1") {
                        grid[nr][nc] = "0";
                        stack.push([nr, nc]);
                    }
                }
            }
        }
    }
    return count;
};
