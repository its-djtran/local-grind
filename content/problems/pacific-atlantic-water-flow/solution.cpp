class Solution {
public:
    vector<vector<int>> pacificAtlantic(vector<vector<int>>& heights) {
        int m = heights.size(), n = heights[0].size();
        vector<pair<int, int>> pacStarts, atlStarts;
        for (int c = 0; c < n; c++) {
            pacStarts.push_back({0, c});
            atlStarts.push_back({m - 1, c});
        }
        for (int r = 0; r < m; r++) {
            pacStarts.push_back({r, 0});
            atlStarts.push_back({r, n - 1});
        }
        auto pacific = climb(heights, pacStarts);
        auto atlantic = climb(heights, atlStarts);
        vector<vector<int>> res;
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++)
                if (pacific[r][c] && atlantic[r][c]) res.push_back({r, c});
        return res;
    }

private:
    vector<vector<char>> climb(const vector<vector<int>>& heights, const vector<pair<int, int>>& starts) {
        int m = heights.size(), n = heights[0].size();
        static const int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        vector<vector<char>> seen(m, vector<char>(n, 0));
        queue<pair<int, int>> q;
        for (auto [r, c] : starts) {
            if (!seen[r][c]) {
                seen[r][c] = 1;
                q.push({r, c});
            }
        }
        while (!q.empty()) {
            auto [r, c] = q.front();
            q.pop();
            int h = heights[r][c];
            for (auto& d : dirs) {
                int nr = r + d[0], nc = c + d[1];
                if (nr >= 0 && nr < m && nc >= 0 && nc < n && !seen[nr][nc] && heights[nr][nc] >= h) {
                    seen[nr][nc] = 1;
                    q.push({nr, nc});
                }
            }
        }
        return seen;
    }
};
