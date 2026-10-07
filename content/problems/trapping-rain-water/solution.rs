impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let (mut lo, mut hi) = (0usize, height.len() - 1);
        let (mut left_max, mut right_max, mut total) = (0, 0, 0);
        while lo < hi {
            left_max = left_max.max(height[lo]);
            right_max = right_max.max(height[hi]);
            if left_max <= right_max {
                total += left_max - height[lo];
                lo += 1;
            } else {
                total += right_max - height[hi];
                hi -= 1;
            }
        }
        total
    }
}
