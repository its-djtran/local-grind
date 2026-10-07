// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

class Solution {
public:
    int firstBadVersion(int n) {
        int lo = 1, hi = n;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2; // avoids overflow near INT_MAX
            if (isBadVersion(mid)) hi = mid; // mid could be the first bad version
            else lo = mid + 1;               // everything up to mid is good
        }
        return lo;
    }
};
