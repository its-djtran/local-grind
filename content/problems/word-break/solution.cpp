class Solution {
public:
    bool wordBreak(string s, vector<string>& wordDict) {
        unordered_set<string> words(wordDict.begin(), wordDict.end());
        size_t longest = 0;
        for (const string& w : wordDict) longest = max(longest, w.size());
        int n = s.size();
        vector<bool> ok(n + 1, false);
        ok[0] = true;
        for (int i = 1; i <= n; i++) {
            for (int j = max(0, i - (int)longest); j < i; j++) {
                if (ok[j] && words.count(s.substr(j, i - j))) {
                    ok[i] = true;
                    break;
                }
            }
        }
        return ok[n];
    }
};
