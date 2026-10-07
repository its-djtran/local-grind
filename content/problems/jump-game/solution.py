class Solution:
    def canJump(self, nums: List[int]) -> bool:
        far, last = 0, len(nums) - 1
        for i, step in enumerate(nums):
            if i > far:
                return False
            if i + step > far:
                far = i + step
                if far >= last:
                    return True
        return True
