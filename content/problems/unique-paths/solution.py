class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        total, k = m + n - 2, min(m, n) - 1
        result = 1
        for i in range(1, k + 1):
            result = result * (total - k + i) // i
        return result
