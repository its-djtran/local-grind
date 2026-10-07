impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        }
        let l = begin_word.len();
        let mut buckets: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
        for (idx, w) in word_list.iter().enumerate() {
            let mut key = w.as_bytes().to_vec();
            for i in 0..l {
                let ch = key[i];
                key[i] = b'*';
                buckets.entry(key.clone()).or_default().push(idx);
                key[i] = ch;
            }
        }
        let mut visited = vec![false; word_list.len()];
        let mut queue: VecDeque<Vec<u8>> = VecDeque::from([begin_word.into_bytes()]);
        let end = end_word.as_bytes();
        let mut steps = 1;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let mut word = queue.pop_front().unwrap();
                if word == end {
                    return steps;
                }
                for i in 0..l {
                    let ch = word[i];
                    word[i] = b'*';
                    let bucket = buckets.remove(&word);
                    word[i] = ch;
                    if let Some(list) = bucket {
                        for nxt in list {
                            if !visited[nxt] {
                                visited[nxt] = true;
                                queue.push_back(word_list[nxt].as_bytes().to_vec());
                            }
                        }
                    }
                }
            }
            steps += 1;
        }
        0
    }
}
