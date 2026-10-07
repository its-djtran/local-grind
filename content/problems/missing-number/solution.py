class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        missing = len(nums)
        for i, x in enumerate(nums):
            missing ^= i ^ x
        return missing
