impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by_key(|iv| iv[0]);
        intervals.windows(2).all(|w| w[1][0] >= w[0][1])
    }
}
