/**
 * @param {character[][]} board
 * @param {string[]} words
 * @return {string[]}
 */
var findWords = function(board, words) {
    const root = {};
    for (const w of words) {
        let node = root;
        for (const ch of w) node = node[ch] ??= {};
        node.$ = w;
    }

    const m = board.length, n = board[0].length;
    const found = [];
    const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];

    const dfs = (i, j, parent) => {
        const ch = board[i][j];
        const node = parent[ch];
        if (node.$ !== undefined) {
            found.push(node.$);
            delete node.$;
        }
        board[i][j] = '#';
        for (const [dx, dy] of dirs) {
            const x = i + dx, y = j + dy;
            if (x >= 0 && x < m && y >= 0 && y < n && node[board[x][y]] !== undefined) {
                dfs(x, y, node);
            }
        }
        board[i][j] = ch;
        if (Object.keys(node).length === 0) delete parent[ch];
    };

    for (let i = 0; i < m; i++)
        for (let j = 0; j < n; j++)
            if (root[board[i][j]] !== undefined) dfs(i, j, root);
    return found;
};
