struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut out = String::new();
        for s in &strs {
            out.push_str(&s.len().to_string());
            out.push('#');
            out.push_str(s);
        }
        out
    }

    fn decode(&self, s: String) -> Vec<String> {
        // Lengths are in bytes, so every slice boundary is a char boundary.
        let b = s.as_bytes();
        let mut out = Vec::new();
        let mut i = 0;
        while i < b.len() {
            let mut j = i;
            while b[j] != b'#' {
                j += 1;
            }
            let n: usize = s[i..j].parse().unwrap();
            out.push(s[j + 1..j + 1 + n].to_string());
            i = j + 1 + n;
        }
        out
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.encode(strs);
 * let ans = obj.decode(data);
 */
