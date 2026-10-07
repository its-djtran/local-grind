impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut t = Vec::with_capacity(2 * s.len() + 3);
        t.push(b'^');
        t.push(b'#');
        for &c in s.as_bytes() {
            t.push(c);
            t.push(b'#');
        }
        t.push(b'$');
        let m = t.len();
        let mut radius = vec![0usize; m];
        let (mut center, mut right) = (0usize, 0usize);
        let (mut best_len, mut best_mid) = (0usize, 0usize);
        for i in 1..m - 1 {
            if i < right {
                radius[i] = (right - i).min(radius[2 * center - i]);
            }
            while t[i + radius[i] + 1] == t[i - radius[i] - 1] {
                radius[i] += 1;
            }
            if i + radius[i] > right {
                center = i;
                right = i + radius[i];
            }
            if radius[i] > best_len {
                best_len = radius[i];
                best_mid = i;
            }
        }
        let start = (best_mid - best_len) / 2;
        s[start..start + best_len].to_string()
    }
}
