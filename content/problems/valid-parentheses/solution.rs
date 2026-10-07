impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for b in s.bytes() {
            match b {
                b'(' | b'[' | b'{' => stack.push(b),
                _ => {
                    let open = match b {
                        b')' => b'(',
                        b']' => b'[',
                        _ => b'{',
                    };
                    if stack.pop() != Some(open) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
