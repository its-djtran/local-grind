/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @return {number[][]}
 */
var insert = function(intervals, newInterval) {
    let [s, e] = newInterval;
    const n = intervals.length;
    const result = [];
    let i = 0;
    while (i < n && intervals[i][1] < s) result.push(intervals[i++]);
    while (i < n && intervals[i][0] <= e) {
        s = Math.min(s, intervals[i][0]);
        e = Math.max(e, intervals[i][1]);
        i++;
    }
    result.push([s, e]);
    while (i < n) result.push(intervals[i++]);
    return result;
};
