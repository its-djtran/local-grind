class Solution {
public:
    string addBinary(string a, string b) {
        int i = a.size() - 1, j = b.size() - 1, carry = 0;
        string res;
        res.reserve(max(a.size(), b.size()) + 1);
        while (i >= 0 || j >= 0 || carry) {
            int total = carry;
            if (i >= 0) total += a[i--] - '0';
            if (j >= 0) total += b[j--] - '0';
            res.push_back(total & 1 ? '1' : '0');
            carry = total >> 1;
        }
        reverse(res.begin(), res.end());
        return res;
    }
};
