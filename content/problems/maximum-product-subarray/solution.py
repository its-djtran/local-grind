class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        hi = lo = best = nums[0]
        for x in nums[1:]:
            a, b = hi * x, lo * x
            hi = max(x, a, b)
            lo = min(x, a, b)
            if hi > best:
                best = hi
        return best
