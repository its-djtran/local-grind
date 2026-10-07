class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        result = [[]]
        for x in nums:
            result += [s + [x] for s in result]
        return result
