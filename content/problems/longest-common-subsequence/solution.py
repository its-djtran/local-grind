class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        if len(text1) < len(text2):
            text1, text2 = text2, text1
        prev = [0] * (len(text2) + 1)
        for ch in text1:
            cur = [0]
            for j, other in enumerate(text2):
                if ch == other:
                    cur.append(prev[j] + 1)
                else:
                    up, left = prev[j + 1], cur[j]
                    cur.append(up if up > left else left)
            prev = cur
        return prev[-1]
