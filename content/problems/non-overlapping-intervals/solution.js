/**
 * @param {number[][]} intervals
 * @return {number}
 */
var eraseOverlapIntervals = function(intervals) {
    intervals.sort((a, b) => a[1] - b[1]);
    let removed = 0;
    let lastEnd = -Infinity;
    for (const [s, e] of intervals) {
        if (s >= lastEnd) lastEnd = e;
        else removed++;
    }
    return removed;
};
