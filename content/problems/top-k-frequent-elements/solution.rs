impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut count: HashMap<i32, usize> = HashMap::new();
        for &x in &nums {
            *count.entry(x).or_insert(0) += 1;
        }
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
        for (value, c) in count {
            buckets[c].push(value);
        }
        let mut result = Vec::with_capacity(k);
        for bucket in buckets.iter().rev() {
            for &value in bucket {
                result.push(value);
                if result.len() == k {
                    return result;
                }
            }
        }
        result
    }
}
