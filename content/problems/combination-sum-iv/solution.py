class Solution:
    def combinationSum4(self, nums: List[int], target: int) -> int:
        nums = sorted(nums)
        dp = [1] + [0] * target
        for t in range(1, target + 1):
            total = 0
            for x in nums:
                if x > t:
                    break
                total += dp[t - x]
            dp[t] = total
        return dp[target]
