impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        points.select_nth_unstable_by_key(k - 1, |p| p[0] * p[0] + p[1] * p[1]);
        points.truncate(k);
        points
    }
}
