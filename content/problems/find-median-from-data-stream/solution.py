class MedianFinder:

    def __init__(self):
        self.lo = []  # max-heap of the smaller half, stored negated
        self.hi = []  # min-heap of the larger half

    def addNum(self, num: int) -> None:
        heapq.heappush(self.hi, -heapq.heappushpop(self.lo, -num))
        if len(self.hi) > len(self.lo):
            heapq.heappush(self.lo, -heapq.heappop(self.hi))

    def findMedian(self) -> float:
        if len(self.lo) > len(self.hi):
            return float(-self.lo[0])
        return (-self.lo[0] + self.hi[0]) / 2
