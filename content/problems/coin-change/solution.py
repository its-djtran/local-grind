class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        INF = amount + 1
        dp = [0] + [INF] * amount
        for c in coins:
            for x in range(c, amount + 1):
                if dp[x - c] + 1 < dp[x]:
                    dp[x] = dp[x - c] + 1
        return dp[amount] if dp[amount] != INF else -1
