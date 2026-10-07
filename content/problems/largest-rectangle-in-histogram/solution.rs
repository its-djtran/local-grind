impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::with_capacity(n + 1);
        let mut best = 0;
        for i in 0..=n {
            let h = if i < n { heights[i] } else { 0 };
            while let Some(&top) = stack.last() {
                if heights[top] < h {
                    break;
                }
                stack.pop();
                let left = stack.last().map_or(-1, |&l| l as i32);
                best = best.max(heights[top] * (i as i32 - left - 1));
            }
            stack.push(i);
        }
        best
    }
}
