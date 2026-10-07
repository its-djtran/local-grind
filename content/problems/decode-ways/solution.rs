impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut prev, mut cur) = (1, if s[0] == b'0' { 0 } else { 1 });
        for i in 1..s.len() {
            let mut next = 0;
            if s[i] != b'0' {
                next += cur;
            }
            if s[i - 1] == b'1' || (s[i - 1] == b'2' && s[i] <= b'6') {
                next += prev;
            }
            (prev, cur) = (cur, next);
        }
        cur
    }
}
