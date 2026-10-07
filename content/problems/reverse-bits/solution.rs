impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut x = n as u32;
        let mut result: u32 = 0;
        for _ in 0..32 {
            result = (result << 1) | (x & 1);
            x >>= 1;
        }
        result as i32
    }
}
