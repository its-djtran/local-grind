class Solution {
public:
    string longestPalindrome(string s) {
        string t = "^#";
        for (char c : s) {
            t += c;
            t += '#';
        }
        t += '$';
        int m = t.size();
        vector<int> radius(m, 0);
        int center = 0, right = 0, bestLen = 0, bestMid = 0;
        for (int i = 1; i < m - 1; i++) {
            if (i < right) radius[i] = min(right - i, radius[2 * center - i]);
            while (t[i + radius[i] + 1] == t[i - radius[i] - 1]) radius[i]++;
            if (i + radius[i] > right) {
                center = i;
                right = i + radius[i];
            }
            if (radius[i] > bestLen) {
                bestLen = radius[i];
                bestMid = i;
            }
        }
        return s.substr((bestMid - bestLen) / 2, bestLen);
    }
};
