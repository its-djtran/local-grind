impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // Invariant: [0, low) are 0s, [low, mid) are 1s, [high, n) are 2s.
        let (mut low, mut mid, mut high) = (0, 0, nums.len());
        while mid < high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                }
                1 => mid += 1,
                _ => {
                    high -= 1;
                    nums.swap(mid, high);
                }
            }
        }
    }
}
