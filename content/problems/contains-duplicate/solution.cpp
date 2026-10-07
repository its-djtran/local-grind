class Solution {
public:
    bool containsDuplicate(vector<int>& nums) {
        unordered_set<int> seen;
        seen.reserve(nums.size() * 2);
        for (int x : nums) {
            if (!seen.insert(x).second) return true;
        }
        return false;
    }
};
