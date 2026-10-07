class Solution {
public:
    int hammingWeight(int n) {
        unsigned int x = n;
        int count = 0;
        while (x != 0) {
            x &= x - 1; // clears the lowest set bit
            ++count;
        }
        return count;
    }
};
