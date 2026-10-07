impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(candidates: &[i32], start: usize, remaining: i32, combo: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if remaining == 0 {
                result.push(combo.clone());
                return;
            }
            for i in start..candidates.len() {
                let c = candidates[i];
                if c > remaining {
                    break;
                }
                combo.push(c);
                backtrack(candidates, i, remaining - c, combo, result);
                combo.pop();
            }
        }

        let mut candidates = candidates;
        candidates.sort_unstable();
        let mut result = Vec::new();
        backtrack(&candidates, 0, target, &mut Vec::new(), &mut result);
        result
    }
}
