class Solution:
    def myAtoi(self, s: str) -> int:
        INT_MAX, INT_MIN = 2**31 - 1, -2**31
        i, n = 0, len(s)
        while i < n and s[i] == " ":
            i += 1
        sign = 1
        if i < n and s[i] in "+-":
            sign = -1 if s[i] == "-" else 1
            i += 1
        value = 0
        while i < n and s[i].isdigit():
            d = ord(s[i]) - ord("0")
            if value > (INT_MAX - d) // 10:
                return INT_MAX if sign == 1 else INT_MIN
            value = value * 10 + d
            i += 1
        return sign * value
