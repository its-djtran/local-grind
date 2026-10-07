class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<vector<int>> result = {{}};
        for (int x : nums) {
            int size = result.size();
            for (int i = 0; i < size; i++) {
                vector<int> s = result[i];
                s.push_back(x);
                result.push_back(move(s));
            }
        }
        return result;
    }
};
