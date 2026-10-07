class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        int n = nums.size();
        vector<vector<int>> res;
        for (int i = 0; i < n - 2; i++) {
            int a = nums[i];
            if (a > 0) break;
            if (i > 0 && a == nums[i - 1]) continue;
            int lo = i + 1, hi = n - 1;
            while (lo < hi) {
                int s = a + nums[lo] + nums[hi];
                if (s < 0) lo++;
                else if (s > 0) hi--;
                else {
                    res.push_back({a, nums[lo], nums[hi]});
                    lo++;
                    hi--;
                    while (lo < hi && nums[lo] == nums[lo - 1]) lo++;
                }
            }
        }
        return res;
    }
};
