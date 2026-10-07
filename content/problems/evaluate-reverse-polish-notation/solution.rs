impl Solution {
    pub fn eval_r_p_n(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i64> = Vec::with_capacity(tokens.len());
        for token in &tokens {
            match token.as_str() {
                op @ ("+" | "-" | "*" | "/") => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(match op {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        _ => a / b,
                    });
                }
                num => stack.push(num.parse().unwrap()),
            }
        }
        stack[0] as i32
    }
}
