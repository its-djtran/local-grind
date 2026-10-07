class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        present = set(nums)
        best = 0
        for x in present:
            if x - 1 not in present:
                y = x + 1
                while y in present:
                    y += 1
                if y - x > best:
                    best = y - x
        return best
