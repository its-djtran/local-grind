impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut starts: Vec<i32> = intervals.iter().map(|iv| iv[0]).collect();
        let mut ends: Vec<i32> = intervals.iter().map(|iv| iv[1]).collect();
        starts.sort_unstable();
        ends.sort_unstable();
        let (mut rooms, mut j) = (0, 0);
        for s in starts {
            if ends[j] <= s {
                j += 1;
            } else {
                rooms += 1;
            }
        }
        rooms
    }
}
