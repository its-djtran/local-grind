impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut unpaired = HashSet::new();
        for ch in s.bytes() {
            if !unpaired.remove(&ch) {
                unpaired.insert(ch);
            }
        }
        // Every character except the unpaired ones forms mirrored pairs; one leftover may sit in the middle.
        (s.len() - unpaired.len() + if unpaired.is_empty() { 0 } else { 1 }) as i32
    }
}
