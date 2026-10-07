class Solution {
public:
    string minWindow(string s, string t) {
        array<int, 128> need{};
        for (char ch : t) need[ch]++;
        int missing = t.size(), left = 0, n = s.size();
        int bestStart = 0, bestLen = n + 1;
        for (int right = 0; right < n; right++) {
            if (need[s[right]]-- > 0) missing--;
            if (missing == 0) {
                // Drop surplus characters from the left; the window stays valid.
                while (need[s[left]] < 0) need[s[left++]]++;
                if (right - left + 1 < bestLen) {
                    bestStart = left;
                    bestLen = right - left + 1;
                }
                // Give up the leftmost required character to look for the next window.
                need[s[left++]]++;
                missing++;
            }
        }
        return bestLen > n ? "" : s.substr(bestStart, bestLen);
    }
};
