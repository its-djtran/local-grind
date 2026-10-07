class Solution {
public:
    int trap(vector<int>& height) {
        int lo = 0, hi = (int)height.size() - 1;
        int leftMax = 0, rightMax = 0, total = 0;
        while (lo <= hi) {
            leftMax = max(leftMax, height[lo]);
            rightMax = max(rightMax, height[hi]);
            if (leftMax <= rightMax) {
                total += leftMax - height[lo++];
            } else {
                total += rightMax - height[hi--];
            }
        }
        return total;
    }
};
