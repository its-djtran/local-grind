class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        unordered_set<int> present(nums.begin(), nums.end());
        int best = 0;
        for (int x : present) {
            if (!present.count(x - 1)) {
                int y = x + 1;
                while (present.count(y)) y++;
                best = max(best, y - x);
            }
        }
        return best;
    }
};
