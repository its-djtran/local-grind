impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut i = 0;
        while i < n && b[i] == b' ' {
            i += 1;
        }
        let mut sign = 1;
        if i < n && (b[i] == b'+' || b[i] == b'-') {
            if b[i] == b'-' {
                sign = -1;
            }
            i += 1;
        }
        let mut value: i32 = 0;
        while i < n && b[i].is_ascii_digit() {
            let d = (b[i] - b'0') as i32;
            if value > (i32::MAX - d) / 10 {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }
            value = value * 10 + d;
            i += 1;
        }
        sign * value
    }
}
