class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        s, e = newInterval
        n, i = len(intervals), 0
        result = []
        while i < n and intervals[i][1] < s:
            result.append(intervals[i])
            i += 1
        while i < n and intervals[i][0] <= e:
            s = min(s, intervals[i][0])
            e = max(e, intervals[i][1])
            i += 1
        result.append([s, e])
        result.extend(intervals[i:])
        return result
