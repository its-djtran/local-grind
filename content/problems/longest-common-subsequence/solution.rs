impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (a, b) = if text1.len() < text2.len() { (text2, text1) } else { (text1, text2) };
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let m = b.len();
        let mut prev = vec![0; m + 1];
        let mut cur = vec![0; m + 1];
        for &ch in a {
            for j in 0..m {
                cur[j + 1] = if ch == b[j] { prev[j] + 1 } else { prev[j + 1].max(cur[j]) };
            }
            std::mem::swap(&mut prev, &mut cur);
        }
        prev[m]
    }
}
