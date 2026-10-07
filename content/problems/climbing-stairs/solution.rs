impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut prev, mut cur) = (1, 1);
        for _ in 1..n {
            (prev, cur) = (cur, prev + cur);
        }
        cur
    }
}
