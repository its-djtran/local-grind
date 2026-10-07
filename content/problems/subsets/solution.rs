impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        for x in nums {
            for i in 0..result.len() {
                let mut s = result[i].clone();
                s.push(x);
                result.push(s);
            }
        }
        result
    }
}
