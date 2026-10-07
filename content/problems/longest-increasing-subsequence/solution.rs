impl Solution {
    pub fn length_of_l_i_s(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = Vec::new();
        for x in nums {
            let k = tails.partition_point(|&t| t < x);
            if k == tails.len() {
                tails.push(x);
            } else {
                tails[k] = x;
            }
        }
        tails.len() as i32
    }
}
