impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let b = s.as_bytes();
        let mut need = [0i32; 128];
        for &c in t.as_bytes() {
            need[c as usize] += 1;
        }
        let mut missing = t.len();
        let mut left = 0usize;
        let (mut best_start, mut best_len) = (0usize, b.len() + 1);
        for right in 0..b.len() {
            let c = b[right] as usize;
            if need[c] > 0 {
                missing -= 1;
            }
            need[c] -= 1;
            if missing == 0 {
                // Drop surplus characters from the left; the window stays valid.
                while need[b[left] as usize] < 0 {
                    need[b[left] as usize] += 1;
                    left += 1;
                }
                if right - left + 1 < best_len {
                    best_start = left;
                    best_len = right - left + 1;
                }
                // Give up the leftmost required character to look for the next window.
                need[b[left] as usize] += 1;
                missing += 1;
                left += 1;
            }
        }
        if best_len > b.len() {
            String::new()
        } else {
            s[best_start..best_start + best_len].to_string()
        }
    }
}
