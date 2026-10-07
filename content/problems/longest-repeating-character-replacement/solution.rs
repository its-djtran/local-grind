impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let mut count = [0usize; 26];
        let (mut left, mut maxf) = (0usize, 0usize);
        for right in 0..s.len() {
            let c = (s[right] - b'A') as usize;
            count[c] += 1;
            maxf = maxf.max(count[c]);
            // The window never shrinks: if it is too costly, slide it right by one instead.
            if right + 1 - left - maxf > k {
                count[(s[left] - b'A') as usize] -= 1;
                left += 1;
            }
        }
        (s.len() - left) as i32
    }
}
