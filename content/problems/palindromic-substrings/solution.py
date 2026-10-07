class Solution:
    def countSubstrings(self, s: str) -> int:
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
        return sum((r + 1) // 2 for r in radius)
