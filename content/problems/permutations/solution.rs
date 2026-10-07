impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], used: &mut [bool], path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                result.push(path.clone());
                return;
            }
            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                used[i] = true;
                path.push(nums[i]);
                backtrack(nums, used, path, result);
                path.pop();
                used[i] = false;
            }
        }

        let mut result = Vec::new();
        backtrack(&nums, &mut vec![false; nums.len()], &mut Vec::new(), &mut result);
        result
    }
}
