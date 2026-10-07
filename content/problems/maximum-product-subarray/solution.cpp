class Solution {
public:
    int maxProduct(vector<int>& nums) {
        int hi = nums[0], lo = nums[0], best = nums[0];
        for (size_t i = 1; i < nums.size(); i++) {
            int x = nums[i];
            int a = hi * x, b = lo * x;
            hi = max({x, a, b});
            lo = min({x, a, b});
            best = max(best, hi);
        }
        return best;
    }
};
