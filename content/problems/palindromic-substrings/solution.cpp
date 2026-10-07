class Solution {
public:
    int countSubstrings(string s) {
        string t = "^#";
        for (char c : s) {
            t += c;
            t += '#';
        }
        t += '$';
        int m = t.size();
        vector<int> radius(m, 0);
        int center = 0, right = 0, count = 0;
        for (int i = 1; i < m - 1; i++) {
            if (i < right) radius[i] = min(right - i, radius[2 * center - i]);
            while (t[i + radius[i] + 1] == t[i - radius[i] - 1]) radius[i]++;
            if (i + radius[i] > right) {
                center = i;
                right = i + radius[i];
            }
            count += (radius[i] + 1) / 2;
        }
        return count;
    }
};
