class Solution {
public:
    bool canJump(vector<int>& nums) {
        int far = 0, last = nums.size() - 1;
        for (int i = 0; i < (int)nums.size(); i++) {
            if (i > far) return false;
            far = max(far, i + nums[i]);
            if (far >= last) return true;
        }
        return true;
    }
};
