class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int low = prices[0], best = 0;
        for (int p : prices) {
            if (p < low) low = p;
            else best = max(best, p - low);
        }
        return best;
    }
};
