impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut x = n as u32;
        let mut count = 0;
        while x != 0 {
            x &= x - 1; // clears the lowest set bit
            count += 1;
        }
        count
    }
}
