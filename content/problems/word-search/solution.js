/**
 * @param {string[][]} board
 * @param {string} word
 * @return {boolean}
 */
var exist = function(board, word) {
    const m = board.length, n = board[0].length;
    if (word.length > m * n) return false;
    const have = new Map();
    for (const row of board) for (const ch of row) have.set(ch, (have.get(ch) || 0) + 1);
    const need = new Map();
    for (const ch of word) need.set(ch, (need.get(ch) || 0) + 1);
    for (const [ch, c] of need) if ((have.get(ch) || 0) < c) return false;
    // Start from the end whose letter is rarer on the board: fewer roots, earlier failures.
    if (have.get(word[0]) > have.get(word[word.length - 1])) word = [...word].reverse().join('');
    const last = word.length - 1;

    const dfs = (i, j, k) => {
        if (board[i][j] !== word[k]) return false;
        if (k === last) return true;
        board[i][j] = '#';
        const found = (i > 0 && dfs(i - 1, j, k + 1))
            || (i + 1 < m && dfs(i + 1, j, k + 1))
            || (j > 0 && dfs(i, j - 1, k + 1))
            || (j + 1 < n && dfs(i, j + 1, k + 1));
        board[i][j] = word[k];
        return found;
    };

    for (let i = 0; i < m; i++)
        for (let j = 0; j < n; j++)
            if (dfs(i, j, 0)) return true;
    return false;
};
