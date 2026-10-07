impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
        let longest = word_dict.iter().map(|w| w.len()).max().unwrap_or(0);
        let n = s.len();
        let mut ok = vec![false; n + 1];
        ok[0] = true;
        for i in 1..=n {
            ok[i] = (i.saturating_sub(longest)..i).any(|j| ok[j] && words.contains(&s[j..i]));
        }
        ok[n]
    }
}
