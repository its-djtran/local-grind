class Solution {
public:
    int rob(vector<int>& nums) {
        int prev = 0, cur = 0;
        for (int x : nums) {
            int next = max(cur, prev + x);
            prev = cur;
            cur = next;
        }
        return cur;
    }
};
