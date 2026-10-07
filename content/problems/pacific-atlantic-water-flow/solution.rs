impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (heights.len(), heights[0].len());
        let mut pac_starts = Vec::with_capacity(m + n);
        let mut atl_starts = Vec::with_capacity(m + n);
        for c in 0..n {
            pac_starts.push((0, c));
            atl_starts.push((m - 1, c));
        }
        for r in 0..m {
            pac_starts.push((r, 0));
            atl_starts.push((r, n - 1));
        }
        let pacific = Self::climb(&heights, &pac_starts);
        let atlantic = Self::climb(&heights, &atl_starts);
        let mut res = Vec::new();
        for r in 0..m {
            for c in 0..n {
                if pacific[r][c] && atlantic[r][c] {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }
        res
    }

    fn climb(heights: &[Vec<i32>], starts: &[(usize, usize)]) -> Vec<Vec<bool>> {
        let (m, n) = (heights.len(), heights[0].len());
        let mut seen = vec![vec![false; n]; m];
        let mut queue = VecDeque::new();
        for &(r, c) in starts {
            if !seen[r][c] {
                seen[r][c] = true;
                queue.push_back((r, c));
            }
        }
        while let Some((r, c)) = queue.pop_front() {
            let h = heights[r][c];
            let neighbors = [
                (r + 1, c),
                (r.wrapping_sub(1), c),
                (r, c + 1),
                (r, c.wrapping_sub(1)),
            ];
            for (nr, nc) in neighbors {
                if nr < m && nc < n && !seen[nr][nc] && heights[nr][nc] >= h {
                    seen[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
        seen
    }
}
