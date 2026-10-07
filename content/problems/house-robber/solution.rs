impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev, mut cur) = (0, 0);
        for x in nums {
            (prev, cur) = (cur, cur.max(prev + x));
        }
        cur
    }
}
