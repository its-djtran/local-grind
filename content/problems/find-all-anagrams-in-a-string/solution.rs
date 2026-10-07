impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (m, n) = (p.len(), s.len());
        if m > n {
            return vec![];
        }
        let mut diff = [0i32; 26]; // window count minus p count, per letter
        for i in 0..m {
            diff[(p[i] - b'a') as usize] -= 1;
            diff[(s[i] - b'a') as usize] += 1;
        }
        let mut bad = diff.iter().filter(|&&d| d != 0).count() as i32; // letters whose counts disagree
        let mut out = Vec::new();
        if bad == 0 {
            out.push(0);
        }
        for i in m..n {
            let (add, rem) = ((s[i] - b'a') as usize, (s[i - m] - b'a') as usize);
            if add != rem {
                let d = diff[add];
                bad += (d == 0) as i32 - (d == -1) as i32;
                diff[add] = d + 1;
                let d = diff[rem];
                bad += (d == 0) as i32 - (d == 1) as i32;
                diff[rem] = d - 1;
            }
            if bad == 0 {
                out.push((i - m + 1) as i32);
            }
        }
        out
    }
}
