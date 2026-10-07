impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let mut best = 0;
        while i < j {
            let w = (j - i) as i32;
            if height[i] < height[j] {
                best = best.max(w * height[i]);
                i += 1;
            } else {
                best = best.max(w * height[j]);
                j -= 1;
            }
        }
        best
    }
}
