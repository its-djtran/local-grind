class Solution:
    def addBinary(self, a: str, b: str) -> str:
        i, j, carry = len(a) - 1, len(b) - 1, 0
        digits = []
        while i >= 0 or j >= 0 or carry:
            total = carry
            if i >= 0:
                total += ord(a[i]) - 48
                i -= 1
            if j >= 0:
                total += ord(b[j]) - 48
                j -= 1
            digits.append("1" if total & 1 else "0")
            carry = total >> 1
        return "".join(reversed(digits))
