class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        low, best = prices[0], 0
        for p in prices:
            if p < low:
                low = p
            elif p - low > best:
                best = p - low
        return best
