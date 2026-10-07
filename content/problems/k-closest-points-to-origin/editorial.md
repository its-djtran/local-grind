Throughout, compare points by **squared** distance `x*x + y*y`. It preserves the ordering of true
distances and stays in exact integer arithmetic.

## Approach 1: Repeated Minimum Selection

**Intuition.** Pick the closest remaining point, remove it, and repeat `k` times.

```python
class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        remaining = list(points)
        result = []
        for _ in range(k):
            best = 0
            for i in range(1, len(remaining)):
                x, y = remaining[i]
                bx, by = remaining[best]
                if x * x + y * y < bx * bx + by * by:
                    best = i
            result.append(remaining.pop(best))
        return result
```

- **Time:** `O(n * k)`, which is quadratic when `k` is a constant fraction of `n`.
- **Space:** `O(n)`.

## Approach 2: Sort by Distance

**Intuition.** Order all points by squared distance and take the prefix of length `k`.

```python
class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        points.sort(key=lambda p: p[0] * p[0] + p[1] * p[1])
        return points[:k]
```

- **Time:** `O(n log n)`.
- **Space:** `O(n)` for the sort keys.

## Approach 3: Max-Heap of Size k

**Intuition.** Scan the points while holding the best `k` seen so far. The only one that can be
displaced is the farthest of them, so store them in a max-heap keyed by distance (a min-heap of
negated distances in Python). Points beyond the current worst are rejected in `O(1)`.

```python
class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        heap = []
        for x, y in points:
            d = -(x * x + y * y)
            if len(heap) < k:
                heapq.heappush(heap, (d, x, y))
            elif d > heap[0][0]:
                heapq.heapreplace(heap, (d, x, y))
        return [[x, y] for _, x, y in heap]
```

- **Time:** `O(n log k)`.
- **Space:** `O(k)` — useful when the points arrive as a stream.

## Approach 4: Quickselect (Optimal on Average)

**Intuition.** We only need the `k` smallest distances as a set, not in order. Quickselect
partitions around a pivot distance into "closer", "equal" and "farther" groups and then
keeps working on just the one group that contains the `k`-th boundary, discarding the rest.

**Algorithm.**

1. Keep a working range of candidates and a count `need` of points still to be chosen.
2. Pick a pivot distance `p` (the median of the first, middle and last candidates' distances, or a
   random one); split candidates into `lo` (`< p`), `eq` (`== p`), `hi`
   (`> p`).
3. If `need <= len(lo)`, continue with `lo`. Otherwise take all of `lo`; if the remainder fits in
   `eq`, take that many from `eq` and stop; otherwise take all of `eq` too and continue with `hi`.

Since the answer is guaranteed to be unique, taking any `need` items from `eq` only happens when
they are all part of it.

```python
class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        cand = [(x * x + y * y, x, y) for x, y in points]
        result = []
        need = k
        while need > 0:
            a, b, c = cand[0][0], cand[len(cand) // 2][0], cand[-1][0]
            p = sorted((a, b, c))[1]  # median of three
            lo = [c for c in cand if c[0] < p]
            eq = [c for c in cand if c[0] == p]
            if need <= len(lo):
                cand = lo
                continue
            result += lo
            need -= len(lo)
            if need <= len(eq):
                result += eq[:need]
                break
            result += eq
            need -= len(eq)
            cand = [c for c in cand if c[0] > p]
        return [[x, y] for _, x, y in result]
```

- **Time:** `O(n)` on average (each round shrinks the range by a constant factor in expectation);
  `O(n^2)` in the unlucky worst case, which a random pivot makes vanishingly unlikely.
- **Space:** `O(n)`.
