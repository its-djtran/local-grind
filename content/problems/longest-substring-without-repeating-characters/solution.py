class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        last = {}
        left = best = 0
        for right, ch in enumerate(s):
            if last.get(ch, -1) >= left:
                left = last[ch] + 1
            last[ch] = right
            if right - left + 1 > best:
                best = right - left + 1
        return best
