class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2:
            return False
        half = total // 2
        bits = 1
        for x in nums:
            bits |= bits << x
        return (bits >> half) & 1 == 1
