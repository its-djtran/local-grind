class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        sort(intervals.begin(), intervals.end(),
             [](const vector<int>& a, const vector<int>& b) { return a[0] < b[0]; });
        vector<vector<int>> merged;
        for (auto& iv : intervals) {
            if (!merged.empty() && iv[0] <= merged.back()[1]) {
                merged.back()[1] = max(merged.back()[1], iv[1]);
            } else {
                merged.push_back(iv);
            }
        }
        return merged;
    }
};
