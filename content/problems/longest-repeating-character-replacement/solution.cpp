class Solution {
public:
    int characterReplacement(string s, int k) {
        array<int, 26> count{};
        int left = 0, maxf = 0, n = s.size();
        for (int right = 0; right < n; right++) {
            maxf = max(maxf, ++count[s[right] - 'A']);
            // The window never shrinks: if it is too costly, slide it right by one instead.
            if (right - left + 1 - maxf > k) {
                count[s[left] - 'A']--;
                left++;
            }
        }
        return n - left;
    }
};
