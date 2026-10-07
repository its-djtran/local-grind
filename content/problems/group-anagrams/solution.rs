impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for word in strs {
            let mut count = [0u8; 26];
            for &c in word.as_bytes() {
                count[(c - b'a') as usize] += 1;
            }
            groups.entry(count).or_default().push(word);
        }
        groups.into_values().collect()
    }
}
