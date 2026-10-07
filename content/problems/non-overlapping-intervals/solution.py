class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        intervals.sort(key=lambda iv: iv[1])
        removed = 0
        last_end = float("-inf")
        for s, e in intervals:
            if s >= last_end:
                last_end = e
            else:
                removed += 1
        return removed
