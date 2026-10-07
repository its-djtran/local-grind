impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut counts = [0i32; 26];
        for ch in magazine.bytes() {
            counts[(ch - b'a') as usize] += 1;
        }
        for ch in ransom_note.bytes() {
            let i = (ch - b'a') as usize;
            if counts[i] == 0 {
                return false;
            }
            counts[i] -= 1;
        }
        true
    }
}
