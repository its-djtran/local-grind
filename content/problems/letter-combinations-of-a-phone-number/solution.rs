impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        const KEYPAD: [&str; 10] = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut combos = vec![String::new()];
        for d in digits.bytes() {
            let letters = KEYPAD[(d - b'0') as usize];
            combos = combos
                .iter()
                .flat_map(|c| letters.chars().map(move |letter| format!("{c}{letter}")))
                .collect();
        }
        combos
    }
}
