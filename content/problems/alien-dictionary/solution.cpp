class Solution {
public:
    string alienOrder(vector<string>& words) {
        vector<array<bool, 26>> adj(26);
        for (auto& row : adj) row.fill(false);
        vector<int> indegree(26, 0);
        array<bool, 26> present{};
        int letters = 0;
        for (auto& w : words)
            for (char ch : w)
                if (!present[ch - 'a']) {
                    present[ch - 'a'] = true;
                    letters++;
                }
        for (size_t i = 0; i + 1 < words.size(); i++) {
            const string& a = words[i];
            const string& b = words[i + 1];
            size_t len = min(a.size(), b.size()), j = 0;
            while (j < len && a[j] == b[j]) j++;
            if (j < len) {
                int x = a[j] - 'a', y = b[j] - 'a';
                if (!adj[x][y]) {
                    adj[x][y] = true;
                    indegree[y]++;
                }
            } else if (a.size() > b.size()) {
                return "";
            }
        }
        queue<int> q;
        for (int c = 0; c < 26; c++)
            if (present[c] && indegree[c] == 0) q.push(c);
        string order;
        while (!q.empty()) {
            int c = q.front();
            q.pop();
            order += char('a' + c);
            for (int d = 0; d < 26; d++)
                if (adj[c][d] && --indegree[d] == 0) q.push(d);
        }
        return (int)order.size() == letters ? order : "";
    }
};
