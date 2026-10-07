impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let present: HashSet<i32> = nums.into_iter().collect();
        let mut best = 0;
        for &x in &present {
            if !present.contains(&(x - 1)) {
                let mut y = x + 1;
                while present.contains(&y) {
                    y += 1;
                }
                best = best.max(y - x);
            }
        }
        best
    }
}
