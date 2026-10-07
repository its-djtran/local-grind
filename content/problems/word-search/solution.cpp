class Solution {
    int m, n, last;
    string w;

    bool dfs(vector<vector<char>>& board, int i, int j, int k) {
        if (board[i][j] != w[k]) return false;
        if (k == last) return true;
        board[i][j] = '#';
        bool found = (i > 0 && dfs(board, i - 1, j, k + 1))
            || (i + 1 < m && dfs(board, i + 1, j, k + 1))
            || (j > 0 && dfs(board, i, j - 1, k + 1))
            || (j + 1 < n && dfs(board, i, j + 1, k + 1));
        board[i][j] = w[k];
        return found;
    }

public:
    bool exist(vector<vector<char>>& board, string word) {
        m = board.size(), n = board[0].size();
        if ((int)word.size() > m * n) return false;
        array<int, 128> have{}, need{};
        for (auto& row : board) for (char ch : row) have[ch]++;
        for (char ch : word) need[ch]++;
        for (int c = 0; c < 128; c++) if (have[c] < need[c]) return false;
        // Start from the end whose letter is rarer on the board: fewer roots, earlier failures.
        if (have[word.front()] > have[word.back()]) reverse(word.begin(), word.end());
        w = std::move(word);
        last = w.size() - 1;
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                if (dfs(board, i, j, 0)) return true;
        return false;
    }
};
