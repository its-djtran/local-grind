impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        let (mut top, mut bottom, mut left, mut right) = (0i32, m - 1, 0i32, n - 1);
        let mut out = Vec::with_capacity((m * n) as usize);
        while top <= bottom && left <= right {
            let (t, b, l, r) = (top as usize, bottom as usize, left as usize, right as usize);
            out.extend_from_slice(&matrix[t][l..=r]);
            for i in t + 1..=b {
                out.push(matrix[i][r]);
            }
            if top < bottom {
                out.extend(matrix[b][l..r].iter().rev());
            }
            if left < right {
                for i in (t + 1..b).rev() {
                    out.push(matrix[i][l]);
                }
            }
            top += 1;
            bottom -= 1;
            left += 1;
            right -= 1;
        }
        out
    }
}
