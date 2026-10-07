class Solution {
public:
    bool canPartition(vector<int>& nums) {
        int total = accumulate(nums.begin(), nums.end(), 0);
        if (total % 2) return false;
        // Max sum is 200 * 100 = 20000, so half is at most 10000.
        bitset<10001> bits;
        bits[0] = 1;
        for (int x : nums) bits |= bits << x;
        return bits[total / 2];
    }
};
