class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        cur = best = nums[0]
        for x in nums[1:]:
            cur = x if cur < 0 else cur + x
            if cur > best:
                best = cur
        return best
