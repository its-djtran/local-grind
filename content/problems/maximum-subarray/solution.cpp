class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int cur = nums[0], best = nums[0];
        for (size_t i = 1; i < nums.size(); i++) {
            cur = cur < 0 ? nums[i] : cur + nums[i];
            best = max(best, cur);
        }
        return best;
    }
};
