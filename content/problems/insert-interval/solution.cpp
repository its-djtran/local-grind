class Solution {
public:
    vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
        int s = newInterval[0], e = newInterval[1];
        int n = intervals.size(), i = 0;
        vector<vector<int>> result;
        result.reserve(n + 1);
        while (i < n && intervals[i][1] < s) result.push_back(intervals[i++]);
        while (i < n && intervals[i][0] <= e) {
            s = min(s, intervals[i][0]);
            e = max(e, intervals[i][1]);
            i++;
        }
        result.push_back({s, e});
        while (i < n) result.push_back(intervals[i++]);
        return result;
    }
};
