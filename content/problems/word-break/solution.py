class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        words = set(wordDict)
        longest = max(map(len, words))
        n = len(s)
        ok = [True] + [False] * n
        for i in range(1, n + 1):
            for j in range(max(0, i - longest), i):
                if ok[j] and s[j:i] in words:
                    ok[i] = True
                    break
        return ok[n]
