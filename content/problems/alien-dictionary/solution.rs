impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut adj = [[false; 26]; 26];
        let mut indegree = [0; 26];
        let mut present = [false; 26];
        for w in &words {
            for b in w.bytes() {
                present[(b - b'a') as usize] = true;
            }
        }
        let letters = present.iter().filter(|&&p| p).count();
        for pair in words.windows(2) {
            let (a, b) = (pair[0].as_bytes(), pair[1].as_bytes());
            match a.iter().zip(b).find(|(x, y)| x != y) {
                Some((&x, &y)) => {
                    let (x, y) = ((x - b'a') as usize, (y - b'a') as usize);
                    if !adj[x][y] {
                        adj[x][y] = true;
                        indegree[y] += 1;
                    }
                }
                None => {
                    if a.len() > b.len() {
                        return String::new();
                    }
                }
            }
        }
        let mut queue: VecDeque<usize> = (0..26).filter(|&c| present[c] && indegree[c] == 0).collect();
        let mut order = String::with_capacity(letters);
        while let Some(c) = queue.pop_front() {
            order.push((b'a' + c as u8) as char);
            for d in 0..26 {
                if adj[c][d] {
                    indegree[d] -= 1;
                    if indegree[d] == 0 {
                        queue.push_back(d);
                    }
                }
            }
        }
        if order.len() == letters { order } else { String::new() }
    }
}
