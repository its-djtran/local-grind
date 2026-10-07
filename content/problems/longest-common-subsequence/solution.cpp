class Solution {
public:
    int longestCommonSubsequence(string text1, string text2) {
        if (text1.size() < text2.size()) swap(text1, text2);
        int m = text2.size();
        vector<int> prev(m + 1, 0), cur(m + 1, 0);
        for (char ch : text1) {
            for (int j = 0; j < m; j++) {
                cur[j + 1] = ch == text2[j] ? prev[j] + 1 : max(prev[j + 1], cur[j]);
            }
            swap(prev, cur);
        }
        return prev[m];
    }
};
