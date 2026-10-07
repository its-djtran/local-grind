class Solution:
    def longestPalindrome(self, s: str) -> str:
        t = "^#" + "#".join(s) + "#$"
        radius = [0] * len(t)
        center = right = 0
        for i in range(1, len(t) - 1):
            if i < right:
                radius[i] = min(right - i, radius[2 * center - i])
            while t[i + radius[i] + 1] == t[i - radius[i] - 1]:
                radius[i] += 1
            if i + radius[i] > right:
                center, right = i, i + radius[i]
        length, mid = max((r, i) for i, r in enumerate(radius))
        start = (mid - length) // 2
        return s[start:start + length]
