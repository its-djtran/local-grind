class Solution:
    def maxArea(self, height: List[int]) -> int:
        i, j = 0, len(height) - 1
        best = 0
        while i < j:
            hi, hj = height[i], height[j]
            if hi < hj:
                area = (j - i) * hi
                i += 1
            else:
                area = (j - i) * hj
                j -= 1
            if area > best:
                best = area
        return best
