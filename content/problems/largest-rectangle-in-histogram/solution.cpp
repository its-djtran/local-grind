class Solution {
public:
    int largestRectangleArea(vector<int>& heights) {
        int n = heights.size();
        vector<int> stack;
        stack.reserve(n + 1);
        int best = 0;
        for (int i = 0; i <= n; i++) {
            int h = i < n ? heights[i] : 0;
            while (!stack.empty() && heights[stack.back()] >= h) {
                int height = heights[stack.back()];
                stack.pop_back();
                int left = stack.empty() ? -1 : stack.back();
                best = max(best, height * (i - left - 1));
            }
            stack.push_back(i);
        }
        return best;
    }
};
