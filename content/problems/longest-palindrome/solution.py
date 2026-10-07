class Solution:
    def longestPalindrome(self, s: str) -> int:
        unpaired = set()
        for ch in s:
            if ch in unpaired:
                unpaired.remove(ch)
            else:
                unpaired.add(ch)
        # Every character except the unpaired ones forms mirrored pairs; one leftover may sit in the middle.
        return len(s) - len(unpaired) + (1 if unpaired else 0)
