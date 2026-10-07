class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        int m = grid.size(), n = grid[0].size();
        static const int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int count = 0;
        vector<pair<int, int>> stack;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] != '1') continue;
                count++;
                grid[i][j] = '0';
                stack.push_back({i, j});
                while (!stack.empty()) {
                    auto [r, c] = stack.back();
                    stack.pop_back();
                    for (auto& d : dirs) {
                        int nr = r + d[0], nc = c + d[1];
                        if (nr >= 0 && nr < m && nc >= 0 && nc < n && grid[nr][nc] == '1') {
                            grid[nr][nc] = '0';
                            stack.push_back({nr, nc});
                        }
                    }
                }
            }
        }
        return count;
    }
};
