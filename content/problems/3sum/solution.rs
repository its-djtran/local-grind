impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = Vec::new();
        for i in 0..n.saturating_sub(2) {
            let a = nums[i];
            if a > 0 {
                break;
            }
            if i > 0 && a == nums[i - 1] {
                continue;
            }
            let (mut lo, mut hi) = (i + 1, n - 1);
            while lo < hi {
                let s = a + nums[lo] + nums[hi];
                match s.cmp(&0) {
                    Ordering::Less => lo += 1,
                    Ordering::Greater => hi -= 1,
                    Ordering::Equal => {
                        res.push(vec![a, nums[lo], nums[hi]]);
                        lo += 1;
                        hi -= 1;
                        while lo < hi && nums[lo] == nums[lo - 1] {
                            lo += 1;
                        }
                    }
                }
            }
        }
        res
    }
}
