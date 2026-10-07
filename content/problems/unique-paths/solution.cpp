class Solution {
public:
    int uniquePaths(int m, int n) {
        int total = m + n - 2, k = min(m, n) - 1;
        long long result = 1;
        for (int i = 1; i <= k; i++) {
            result = result * (total - k + i) / i;
        }
        return result;
    }
};
