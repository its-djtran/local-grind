// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut lo, mut hi) = (1, n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2; // avoids overflow near i32::MAX
            if self.isBadVersion(mid) {
                hi = mid; // mid could be the first bad version
            } else {
                lo = mid + 1; // everything up to mid is good
            }
        }
        lo
    }
}
