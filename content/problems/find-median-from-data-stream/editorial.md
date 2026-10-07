## Approach 1: Sort on Every Query

**Intuition.** Store the numbers in a list; whenever a median is requested, sort and read the
middle.

```python
class MedianFinder:

    def __init__(self):
        self.nums = []

    def addNum(self, num: int) -> None:
        self.nums.append(num)

    def findMedian(self) -> float:
        s = sorted(self.nums)
        n = len(s)
        if n % 2:
            return float(s[n // 2])
        return (s[n // 2 - 1] + s[n // 2]) / 2
```

- **Time:** `O(1)` per `addNum`, `O(n log n)` per `findMedian` — quadratic overall when queries
  are frequent.
- **Space:** `O(n)`.

## Approach 2: Keep the List Sorted

**Intuition.** Rather than sorting at query time, insert each number at its correct position.
Binary search finds the position in `O(log n)`, but shifting the tail of the list to make room
still costs `O(n)`. Queries become `O(1)`.

```python
class MedianFinder:

    def __init__(self):
        self.nums = []

    def addNum(self, num: int) -> None:
        insort(self.nums, num)

    def findMedian(self) -> float:
        s, n = self.nums, len(self.nums)
        if n % 2:
            return float(s[n // 2])
        return (s[n // 2 - 1] + s[n // 2]) / 2
```

- **Time:** `O(n)` per `addNum` (a fast memory move in practice), `O(1)` per `findMedian`.
- **Space:** `O(n)`.

## Approach 3: Two Heaps (Optimal)

**Intuition.** The median only depends on the boundary between the smaller half and the larger
half of the numbers. Keep the smaller half in a **max-heap** `lo` and the larger half in a
**min-heap** `hi`, with every element of `lo` at most every element of `hi`, and sizes balanced
so that `len(lo) == len(hi)` or `len(lo) == len(hi) + 1`. Then:

- odd count: the median is the top of `lo`;
- even count: the median is the average of the tops of `lo` and `hi`.

**Algorithm (addNum).** Python only has min-heaps, so `lo` stores negated values.

1. Push `num` onto `lo`, then move `lo`'s largest element to `hi`. This keeps every element of
   `lo` no larger than every element of `hi`, whichever half `num` belongs to.
2. If `hi` now has more elements than `lo`, move `hi`'s smallest element back to `lo`.

```python
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
```

- **Time:** `O(log n)` per `addNum`, `O(1)` per `findMedian`.
- **Space:** `O(n)`.

## Follow-up: Bounded Values

If every number is in `[0, 100]`, keep an array `count[0..100]` and the total `n`. `addNum` is
`O(1)`; `findMedian` walks the 101 buckets to find the values at ranks `(n - 1) // 2` and
`n // 2`, which is constant time for a fixed range.

If only `99%` of numbers fall in `[0, 100]`, keep the buckets for that range plus two counters
(or small sorted containers) for values below `0` and above `100`. The median almost always
lands inside the bucket range, so the outliers only need to be counted; in the rare case the
median rank falls among them, look it up in the corresponding container.
