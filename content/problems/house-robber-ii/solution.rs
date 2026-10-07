impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        fn line(houses: &[i32]) -> i32 {
            let (mut prev, mut cur) = (0, 0);
            for &x in houses {
                (prev, cur) = (cur, cur.max(prev + x));
            }
            cur
        }
        line(&nums[..n - 1]).max(line(&nums[1..]))
    }
}
