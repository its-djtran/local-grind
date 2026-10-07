impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let last = nums.len() - 1;
        let mut far = 0;
        for (i, &step) in nums.iter().enumerate() {
            if i > far {
                return false;
            }
            far = far.max(i + step as usize);
            if far >= last {
                return true;
            }
        }
        true
    }
}
