impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut count = [0i32; 26];
        for &c in s.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }
        for &c in t.as_bytes() {
            let i = (c - b'a') as usize;
            count[i] -= 1;
            if count[i] < 0 {
                return false;
            }
        }
        true
    }
}
