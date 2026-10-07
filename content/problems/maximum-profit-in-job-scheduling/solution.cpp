class Solution {
public:
    int jobScheduling(vector<int>& startTime, vector<int>& endTime, vector<int>& profit) {
        int n = startTime.size();
        vector<array<int, 3>> jobs(n);
        for (int i = 0; i < n; i++) jobs[i] = {endTime[i], startTime[i], profit[i]};
        sort(jobs.begin(), jobs.end());
        vector<int> ends(n);
        for (int i = 0; i < n; i++) ends[i] = jobs[i][0];
        vector<int> dp(n + 1, 0);
        for (int i = 1; i <= n; i++) {
            auto [e, s, p] = jobs[i - 1];
            int j = upper_bound(ends.begin(), ends.begin() + i - 1, s) - ends.begin();
            dp[i] = max(dp[i - 1], dp[j] + p);
        }
        return dp[n];
    }
};
