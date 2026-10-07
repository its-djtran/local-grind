class Solution {
public:
    int getSum(int a, int b) {
        // Unsigned arithmetic makes the carry shift well-defined.
        unsigned int x = a, y = b;
        while (y != 0) {
            unsigned int carry = (x & y) << 1;
            x ^= y;
            y = carry;
        }
        return static_cast<int>(x);
    }
};
