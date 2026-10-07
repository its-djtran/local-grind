## Approach 1: Repeatedly Pick the Most Frequent

Count every value, then `k` times scan the counts for the most frequent remaining value and remove
it.

```python
class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count = Counter(nums)
        result = []
        for _ in range(k):
            best = max(count, key=count.get)
            result.append(best)
            del count[best]
        return result
```

- **Time:** `O(n + k * u)` where `u` is the number of distinct values — quadratic when both `k`
  and `u` are in the tens of thousands.
- **Space:** `O(u)`.

## Approach 2: Sort by Frequency

Count, then sort the distinct values by decreasing count and take the first `k`.

```python
class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count = Counter(nums)
        return sorted(count, key=count.get, reverse=True)[:k]
```

- **Time:** `O(n + u log u)`.
- **Space:** `O(u)`.

## Approach 3: Min-Heap of Size k

Keep the `k` best `(count, value)` pairs seen so far in a min-heap; when it grows past `k`, pop the
smallest. What remains are the `k` most frequent values. (`heapq.nlargest` does exactly this.)

```python
class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count = Counter(nums)
        heap = []
        for value, c in count.items():
            heappush(heap, (c, value))
            if len(heap) > k:
                heappop(heap)
        return [value for _, value in heap]
```

- **Time:** `O(n + u log k)`.
- **Space:** `O(u + k)`.

## Approach 4: Bucket Sort by Count (Optimal)

**Intuition.** A count is an integer between `1` and `n`, so instead of comparison sorting we can
drop each value into a bucket indexed by its count. Walking the buckets from count `n` downwards
visits values in order of decreasing frequency.

**Algorithm.**

1. Count occurrences with a hash map.
2. Create `n + 1` empty buckets; append each value to `buckets[count]`.
3. Walk from bucket `n` down to `1`, collecting values until `k` have been gathered. Because the
   answer is unique, the bucket where we stop never has to be split arbitrarily.

```python
class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count = Counter(nums)
        buckets = [[] for _ in range(len(nums) + 1)]
        for value, c in count.items():
            buckets[c].append(value)
        result = []
        for c in range(len(nums), 0, -1):
            for value in buckets[c]:
                result.append(value)
                if len(result) == k:
                    return result
        return result
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the counts and buckets.
