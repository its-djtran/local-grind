impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        let mut word: Vec<char> = word.chars().collect();
        if word.len() > m * n {
            return false;
        }
        let mut have = [0usize; 128];
        let mut need = [0usize; 128];
        for row in &board {
            for &ch in row {
                have[ch as usize] += 1;
            }
        }
        for &ch in &word {
            need[ch as usize] += 1;
        }
        if (0..128).any(|c| have[c] < need[c]) {
            return false;
        }
        // Start from the end whose letter is rarer on the board: fewer roots, earlier failures.
        if have[word[0] as usize] > have[word[word.len() - 1] as usize] {
            word.reverse();
        }

        fn dfs(board: &mut Vec<Vec<char>>, word: &[char], i: usize, j: usize, k: usize) -> bool {
            if board[i][j] != word[k] {
                return false;
            }
            if k == word.len() - 1 {
                return true;
            }
            let (m, n) = (board.len(), board[0].len());
            board[i][j] = '#';
            let found = (i > 0 && dfs(board, word, i - 1, j, k + 1))
                || (i + 1 < m && dfs(board, word, i + 1, j, k + 1))
                || (j > 0 && dfs(board, word, i, j - 1, k + 1))
                || (j + 1 < n && dfs(board, word, i, j + 1, k + 1));
            board[i][j] = word[k];
            found
        }

        for i in 0..m {
            for j in 0..n {
                if dfs(&mut board, &word, i, j, 0) {
                    return true;
                }
            }
        }
        false
    }
}
