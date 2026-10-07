class Solution {
public:
    int calculate(string s) {
        long long total = 0, num = 0;
        int sign = 1;
        vector<pair<long long, int>> stack;
        for (char ch : s) {
            if (isdigit((unsigned char)ch)) {
                num = num * 10 + (ch - '0');
            } else if (ch == '+' || ch == '-') {
                total += sign * num;
                num = 0;
                sign = ch == '+' ? 1 : -1;
            } else if (ch == '(') {
                stack.push_back({total, sign});
                total = 0;
                sign = 1;
            } else if (ch == ')') {
                total += sign * num;
                num = 0;
                auto [prev, prevSign] = stack.back();
                stack.pop_back();
                total = prev + prevSign * total;
            }
        }
        return (int)(total + sign * num);
    }
};
