impl Solution {
    pub fn count_substrings(s: String) -> i32 {
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
        let mut count = 0usize;
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
            count += (radius[i] + 1) / 2;
        }
        count as i32
    }
}
