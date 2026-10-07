class Solution {
public:
    int reverseBits(int n) {
        uint32_t x = n, result = 0;
        for (int i = 0; i < 32; ++i) {
            result = (result << 1) | (x & 1);
            x >>= 1;
        }
        return static_cast<int>(result);
    }
};
