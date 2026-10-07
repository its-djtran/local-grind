impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut s, mut e) = (new_interval[0], new_interval[1]);
        let mut result = Vec::with_capacity(intervals.len() + 1);
        let mut it = intervals.into_iter().peekable();
        while let Some(iv) = it.next_if(|iv| iv[1] < s) {
            result.push(iv);
        }
        while let Some(iv) = it.next_if(|iv| iv[0] <= e) {
            s = s.min(iv[0]);
            e = e.max(iv[1]);
        }
        result.push(vec![s, e]);
        result.extend(it);
        result
    }
}
