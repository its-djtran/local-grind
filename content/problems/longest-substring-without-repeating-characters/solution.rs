impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = [usize::MAX; 256];
        let (mut left, mut best) = (0usize, 0usize);
        for (right, &b) in s.as_bytes().iter().enumerate() {
            let c = b as usize;
            if last[c] != usize::MAX && last[c] >= left {
                left = last[c] + 1;
            }
            last[c] = right;
            best = best.max(right - left + 1);
        }
        best as i32
    }
}
