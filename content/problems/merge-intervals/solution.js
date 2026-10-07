/**
 * @param {number[][]} intervals
 * @return {number[][]}
 */
var merge = function(intervals) {
    intervals.sort((a, b) => a[0] - b[0]);
    const merged = [];
    for (const [s, e] of intervals) {
        const last = merged[merged.length - 1];
        if (last && s <= last[1]) {
            if (e > last[1]) last[1] = e;
        } else {
            merged.push([s, e]);
        }
    }
    return merged;
};
