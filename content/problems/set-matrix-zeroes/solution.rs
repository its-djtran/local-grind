impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let first_row = matrix[0].iter().any(|&v| v == 0);
        let first_col = matrix.iter().any(|row| row[0] == 0);
        let (head, rest) = matrix.split_at_mut(1);
        let top = &mut head[0];
        for row in rest.iter_mut() {
            for j in 1..n {
                if row[j] == 0 {
                    row[0] = 0;
                    top[j] = 0;
                }
            }
        }
        for row in rest.iter_mut() {
            if row[0] == 0 {
                row[1..].fill(0);
            } else {
                for j in 1..n {
                    if top[j] == 0 {
                        row[j] = 0;
                    }
                }
            }
        }
        if first_row {
            top.fill(0);
        }
        if first_col {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}
