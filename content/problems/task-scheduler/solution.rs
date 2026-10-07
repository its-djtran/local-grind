impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = [0i32; 26];
        for &t in &tasks {
            counts[(t as u8 - b'A') as usize] += 1;
        }
        let f = *counts.iter().max().unwrap();
        let k = counts.iter().filter(|&&c| c == f).count() as i32;
        (tasks.len() as i32).max((f - 1) * (n + 1) + k)
    }
}
