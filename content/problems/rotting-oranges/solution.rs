impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::new();
        let mut fresh = 0;
        for r in 0..m {
            for c in 0..n {
                match grid[r][c] {
                    2 => queue.push_back((r, c)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }
        let mut minutes = 0;
        while !queue.is_empty() && fresh > 0 {
            for _ in 0..queue.len() {
                let (r, c) = queue.pop_front().unwrap();
                let neighbors = [
                    (r + 1, c),
                    (r.wrapping_sub(1), c),
                    (r, c + 1),
                    (r, c.wrapping_sub(1)),
                ];
                for (nr, nc) in neighbors {
                    if nr < m && nc < n && grid[nr][nc] == 1 {
                        grid[nr][nc] = 2;
                        fresh -= 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
            minutes += 1;
        }
        if fresh == 0 { minutes } else { -1 }
    }
}
