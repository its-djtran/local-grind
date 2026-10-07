class Solution {
public:
    int maxArea(vector<int>& height) {
        int i = 0, j = height.size() - 1, best = 0;
        while (i < j) {
            if (height[i] < height[j]) {
                best = max(best, (j - i) * height[i]);
                i++;
            } else {
                best = max(best, (j - i) * height[j]);
                j--;
            }
        }
        return best;
    }
};
