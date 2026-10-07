class Solution {
    struct Node {
        Node* children[26] = {};
        int count = 0;
        const string* word = nullptr;
    };

    int m, n;
    vector<string> found;

    void dfs(vector<vector<char>>& board, int i, int j, Node* parent) {
        char ch = board[i][j];
        Node* node = parent->children[ch - 'a'];
        if (node->word) {
            found.push_back(*node->word);
            node->word = nullptr;
        }
        board[i][j] = '#';
        static constexpr int dirs[5] = {-1, 0, 1, 0, -1};
        for (int d = 0; d < 4; d++) {
            int x = i + dirs[d], y = j + dirs[d + 1];
            if (x >= 0 && x < m && y >= 0 && y < n && board[x][y] != '#' && node->children[board[x][y] - 'a'])
                dfs(board, x, y, node);
        }
        board[i][j] = ch;
        if (!node->count && !node->word) {
            parent->children[ch - 'a'] = nullptr;
            parent->count--;
        }
    }

public:
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        Node* root = new Node();
        for (const string& w : words) {
            Node* node = root;
            for (char ch : w) {
                Node*& next = node->children[ch - 'a'];
                if (!next) {
                    next = new Node();
                    node->count++;
                }
                node = next;
            }
            node->word = &w;
        }

        m = board.size(), n = board[0].size();
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                if (root->children[board[i][j] - 'a'])
                    dfs(board, i, j, root);
        return found;
    }
};
