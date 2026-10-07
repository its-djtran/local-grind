struct Node {
    children: [usize; 26],
    count: u8,
    word: Option<usize>,
}

impl Node {
    fn new() -> Self {
        Node { children: [0; 26], count: 0, word: None }
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        // Arena trie: index 0 is the root, so a child index of 0 means "no child".
        let mut trie = vec![Node::new()];
        for (idx, w) in words.iter().enumerate() {
            let mut node = 0;
            for b in w.bytes() {
                let c = (b - b'a') as usize;
                if trie[node].children[c] == 0 {
                    trie.push(Node::new());
                    let next = trie.len() - 1;
                    trie[node].children[c] = next;
                    trie[node].count += 1;
                }
                node = trie[node].children[c];
            }
            trie[node].word = Some(idx);
        }

        let mut board: Vec<Vec<u8>> = board.into_iter().map(|row| row.into_iter().map(|c| c as u8).collect()).collect();
        let (m, n) = (board.len(), board[0].len());
        let mut found = Vec::new();

        fn dfs(board: &mut Vec<Vec<u8>>, trie: &mut Vec<Node>, found: &mut Vec<usize>, i: usize, j: usize, parent: usize) {
            let ch = board[i][j];
            let c = (ch - b'a') as usize;
            let node = trie[parent].children[c];
            if let Some(idx) = trie[node].word.take() {
                found.push(idx);
            }
            board[i][j] = b'#';
            let (m, n) = (board.len(), board[0].len());
            for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let (x, y) = (i as i32 + dx, j as i32 + dy);
                if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                let b = board[x][y];
                if b != b'#' && trie[node].children[(b - b'a') as usize] != 0 {
                    dfs(board, trie, found, x, y, node);
                }
            }
            board[i][j] = ch;
            if trie[node].count == 0 && trie[node].word.is_none() {
                trie[parent].children[c] = 0;
                trie[parent].count -= 1;
            }
        }

        for i in 0..m {
            for j in 0..n {
                if trie[0].children[(board[i][j] - b'a') as usize] != 0 {
                    dfs(&mut board, &mut trie, &mut found, i, j, 0);
                }
            }
        }
        found.into_iter().map(|idx| words[idx].clone()).collect()
    }
}
