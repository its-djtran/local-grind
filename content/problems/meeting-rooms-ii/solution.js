/**
 * @param {number[][]} intervals
 * @return {number}
 */
var minMeetingRooms = function(intervals) {
    const n = intervals.length;
    const starts = new Int32Array(n), ends = new Int32Array(n);
    for (let i = 0; i < n; i++) {
        starts[i] = intervals[i][0];
        ends[i] = intervals[i][1];
    }
    starts.sort();
    ends.sort();
    let rooms = 0, j = 0;
    for (let i = 0; i < n; i++) {
        if (ends[j] <= starts[i]) j++;
        else rooms++;
    }
    return rooms;
};
