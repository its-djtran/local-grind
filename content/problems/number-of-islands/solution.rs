impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut count = 0;
        let mut stack = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != '1' {
                    continue;
                }
                count += 1;
                grid[i][j] = '0';
                stack.push((i, j));
                while let Some((r, c)) = stack.pop() {
                    let neighbors = [
                        (r + 1, c),
                        (r.wrapping_sub(1), c),
                        (r, c + 1),
                        (r, c.wrapping_sub(1)),
                    ];
                    for (nr, nc) in neighbors {
                        if nr < m && nc < n && grid[nr][nc] == '1' {
                            grid[nr][nc] = '0';
                            stack.push((nr, nc));
                        }
                    }
                }
            }
        }
        count
    }
}
