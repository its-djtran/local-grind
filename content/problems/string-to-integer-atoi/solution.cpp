class Solution {
public:
    int myAtoi(string s) {
        int i = 0, n = s.size();
        while (i < n && s[i] == ' ') i++;
        int sign = 1;
        if (i < n && (s[i] == '+' || s[i] == '-')) {
            if (s[i] == '-') sign = -1;
            i++;
        }
        int value = 0;
        while (i < n && isdigit((unsigned char)s[i])) {
            int d = s[i] - '0';
            if (value > (INT_MAX - d) / 10) return sign == 1 ? INT_MAX : INT_MIN;
            value = value * 10 + d;
            i++;
        }
        return sign * value;
    }
};
