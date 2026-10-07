impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|iv| iv[1]);
        let mut removed = 0;
        let mut last_end = i32::MIN;
        for iv in &intervals {
            if iv[0] >= last_end {
                last_end = iv[1];
            } else {
                removed += 1;
            }
        }
        removed
    }
}
