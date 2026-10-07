class Solution {
    vector<vector<int>> result;
    vector<int> combo;

    void backtrack(const vector<int>& candidates, int start, int remaining) {
        if (remaining == 0) {
            result.push_back(combo);
            return;
        }
        for (int i = start; i < (int)candidates.size(); i++) {
            int c = candidates[i];
            if (c > remaining) break;
            combo.push_back(c);
            backtrack(candidates, i, remaining - c);
            combo.pop_back();
        }
    }

public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());
        backtrack(candidates, 0, target);
        return result;
    }
};
