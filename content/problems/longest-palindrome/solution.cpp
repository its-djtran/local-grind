class Solution {
public:
    int longestPalindrome(string s) {
        // Parity bit per ASCII character; true means currently unpaired.
        bool unpaired[128] = {false};
        int odd = 0;
        for (unsigned char ch : s) {
            unpaired[ch] = !unpaired[ch];
            odd += unpaired[ch] ? 1 : -1;
        }
        // Every character except the unpaired ones forms mirrored pairs; one leftover may sit in the middle.
        return (int)s.size() - odd + (odd > 0 ? 1 : 0);
    }
};
