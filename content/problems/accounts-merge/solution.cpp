class Solution {
public:
    vector<vector<string>> accountsMerge(vector<vector<string>>& accounts) {
        unordered_map<string, int> id;
        vector<const string*> emails;
        vector<int> owner;
        for (int a = 0; a < (int)accounts.size(); a++) {
            auto& acc = accounts[a];
            for (int i = 1; i < (int)acc.size(); i++) {
                auto [it, inserted] = id.try_emplace(acc[i], (int)emails.size());
                if (inserted) {
                    emails.push_back(&it->first);
                    owner.push_back(a);
                    parent.push_back(it->second);
                }
            }
            int root = find(id[acc[1]]);
            for (int i = 2; i < (int)acc.size(); i++) {
                int other = find(id[acc[i]]);
                if (other != root) parent[other] = root;
            }
        }
        unordered_map<int, vector<string>> groups;
        for (int e = 0; e < (int)emails.size(); e++) groups[find(e)].push_back(*emails[e]);
        vector<vector<string>> res;
        res.reserve(groups.size());
        for (auto& [root, list] : groups) {
            sort(list.begin(), list.end());
            vector<string> row;
            row.reserve(list.size() + 1);
            row.push_back(accounts[owner[root]][0]);
            for (auto& s : list) row.push_back(move(s));
            res.push_back(move(row));
        }
        return res;
    }

private:
    vector<int> parent;

    int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }
};
