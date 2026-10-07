/**
 * @param {number[][]} image
 * @param {number} sr
 * @param {number} sc
 * @param {number} color
 * @return {number[][]}
 */
var floodFill = function(image, sr, sc, color) {
    const original = image[sr][sc];
    if (original === color) return image;
    const m = image.length, n = image[0].length;
    const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    image[sr][sc] = color;
    const queue = [[sr, sc]];
    for (let head = 0; head < queue.length; head++) {
        const [r, c] = queue[head];
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < m && nc >= 0 && nc < n && image[nr][nc] === original) {
                image[nr][nc] = color;
                queue.push([nr, nc]);
            }
        }
    }
    return image;
};
