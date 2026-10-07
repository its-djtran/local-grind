class Solution:
    def climbStairs(self, n: int) -> int:
        prev, cur = 1, 1
        for _ in range(n - 1):
            prev, cur = cur, prev + cur
        return cur
