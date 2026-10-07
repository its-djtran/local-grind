impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (mut total, mut sign, mut num) = (0i64, 1i64, 0i64);
        let mut stack: Vec<(i64, i64)> = Vec::new();
        for b in s.bytes() {
            match b {
                b'0'..=b'9' => num = num * 10 + (b - b'0') as i64,
                b'+' | b'-' => {
                    total += sign * num;
                    num = 0;
                    sign = if b == b'+' { 1 } else { -1 };
                }
                b'(' => {
                    stack.push((total, sign));
                    total = 0;
                    sign = 1;
                }
                b')' => {
                    total += sign * num;
                    num = 0;
                    let (prev, prev_sign) = stack.pop().unwrap();
                    total = prev + prev_sign * total;
                }
                _ => {}
            }
        }
        (total + sign * num) as i32
    }
}
