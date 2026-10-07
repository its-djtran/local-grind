class Solution {
public:
    vector<int> spiralOrder(vector<vector<int>>& matrix) {
        int top = 0, bottom = matrix.size() - 1;
        int left = 0, right = matrix[0].size() - 1;
        vector<int> out;
        out.reserve(matrix.size() * matrix[0].size());
        while (top <= bottom && left <= right) {
            for (int j = left; j <= right; j++) out.push_back(matrix[top][j]);
            for (int i = top + 1; i <= bottom; i++) out.push_back(matrix[i][right]);
            if (top < bottom) for (int j = right - 1; j >= left; j--) out.push_back(matrix[bottom][j]);
            if (left < right) for (int i = bottom - 1; i > top; i--) out.push_back(matrix[i][left]);
            top++, bottom--, left++, right--;
        }
        return out;
    }
};
