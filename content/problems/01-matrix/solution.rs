impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut dist = vec![vec![-1; n]; m];
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    dist[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }
        while let Some((r, c)) = queue.pop_front() {
            let d = dist[r][c] + 1;
            let neighbors = [
                (r + 1, c),
                (r.wrapping_sub(1), c),
                (r, c + 1),
                (r, c.wrapping_sub(1)),
            ];
            for (nr, nc) in neighbors {
                if nr < m && nc < n && dist[nr][nc] == -1 {
                    dist[nr][nc] = d;
                    queue.push_back((nr, nc));
                }
            }
        }
        dist
    }
}
