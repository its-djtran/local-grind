class Solution {
public:
    bool isAnagram(string s, string t) {
        if (s.size() != t.size()) return false;
        array<int, 26> count{};
        for (char ch : s) count[ch - 'a']++;
        for (char ch : t)
            if (--count[ch - 'a'] < 0) return false;
        return true;
    }
};
