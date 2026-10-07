class Solution:
    def getSum(self, a: int, b: int) -> int:
        mask = 0xFFFFFFFF
        a &= mask
        b &= mask
        while b:
            a, b = (a ^ b) & mask, ((a & b) << 1) & mask
        # Bit 31 set means the 32-bit pattern encodes a negative number.
        return a if a <= 0x7FFFFFFF else ~(a ^ mask)
