class Solution:
    def trap(self, height: List[int]) -> int:
        lo, hi = 0, len(height) - 1
        left_max = right_max = total = 0
        while lo <= hi:
            left_max = max(left_max, height[lo])
            right_max = max(right_max, height[hi])
            if left_max <= right_max:
                total += left_max - height[lo]
                lo += 1
            else:
                total += right_max - height[hi]
                hi -= 1
        return total
