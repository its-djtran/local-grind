class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        unordered_map<string, int> index;
        vector<vector<string>> groups;
        for (auto& word : strs) {
            string key(26, '\0');
            for (char ch : word) key[ch - 'a']++;
            auto [it, inserted] = index.try_emplace(std::move(key), groups.size());
            if (inserted) groups.emplace_back();
            groups[it->second].push_back(word);
        }
        return groups;
    }
};
