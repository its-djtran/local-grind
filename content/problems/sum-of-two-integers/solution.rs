impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        // Work on the raw 32-bit pattern; bits shifted out of the top are discarded.
        let (mut x, mut y) = (a as u32, b as u32);
        while y != 0 {
            let carry = (x & y) << 1;
            x ^= y;
            y = carry;
        }
        x as i32
    }
}
