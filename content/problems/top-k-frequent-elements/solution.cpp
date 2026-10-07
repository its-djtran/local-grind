class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> count;
        for (int x : nums) count[x]++;
        int n = nums.size();
        vector<vector<int>> buckets(n + 1);
        for (auto& [value, c] : count) buckets[c].push_back(value);
        vector<int> result;
        result.reserve(k);
        for (int c = n; c > 0; c--) {
            for (int value : buckets[c]) {
                result.push_back(value);
                if ((int)result.size() == k) return result;
            }
        }
        return result;
    }
};
