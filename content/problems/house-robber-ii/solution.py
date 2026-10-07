class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1:
            return nums[0]

        def line(lo: int, hi: int) -> int:
            prev = cur = 0
            for i in range(lo, hi):
                prev, cur = cur, max(cur, prev + nums[i])
            return cur

        return max(line(0, n - 1), line(1, n))
