class Solution {
public:
    int numDecodings(string s) {
        int prev = 1, cur = s[0] == '0' ? 0 : 1;
        for (int i = 1; i < (int)s.size(); i++) {
            int next = 0;
            if (s[i] != '0') next += cur;
            if (s[i - 1] == '1' || (s[i - 1] == '2' && s[i] <= '6')) next += prev;
            prev = cur;
            cur = next;
        }
        return cur;
    }
};
