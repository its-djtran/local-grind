impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let (mut i, mut j) = (a.len(), b.len());
        let mut carry = 0u8;
        let mut digits = Vec::with_capacity(a.len().max(b.len()) + 1);
        while i > 0 || j > 0 || carry > 0 {
            let mut total = carry;
            if i > 0 {
                i -= 1;
                total += a[i] - b'0';
            }
            if j > 0 {
                j -= 1;
                total += b[j] - b'0';
            }
            digits.push(b'0' + (total & 1));
            carry = total >> 1;
        }
        digits.reverse();
        String::from_utf8(digits).unwrap()
    }
}
