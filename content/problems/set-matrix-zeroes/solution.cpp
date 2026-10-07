class Solution {
public:
    void setZeroes(vector<vector<int>>& matrix) {
        int m = matrix.size(), n = matrix[0].size();
        auto& top = matrix[0];
        bool firstRow = ranges::find(top, 0) != top.end();
        bool firstCol = false;
        for (int i = 0; i < m; i++) if (matrix[i][0] == 0) firstCol = true;
        for (int i = 1; i < m; i++) {
            auto& row = matrix[i];
            for (int j = 1; j < n; j++) {
                if (row[j] == 0) {
                    row[0] = 0;
                    top[j] = 0;
                }
            }
        }
        for (int i = 1; i < m; i++) {
            auto& row = matrix[i];
            if (row[0] == 0) {
                fill(row.begin() + 1, row.end(), 0);
            } else {
                for (int j = 1; j < n; j++) if (top[j] == 0) row[j] = 0;
            }
        }
        if (firstRow) fill(top.begin(), top.end(), 0);
        if (firstCol) for (int i = 0; i < m; i++) matrix[i][0] = 0;
    }
};
