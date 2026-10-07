## Approach 1: Brute Force

Try every pair `(i, j)` with `i < j` and return the first pair whose values add up to `target`.

```python
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                if nums[i] + nums[j] == target:
                    return [i, j]
```

- **Time:** `O(n^2)` — every pair is examined in the worst case.
- **Space:** `O(1)`.

## Approach 2: One-Pass Hash Map (Optimal)

**Intuition.** While scanning left to right, the only question for the current value `x` is
"have I already seen `target - x`?". A hash map answers that in expected constant time.

**Algorithm.**

1. Create an empty map `seen` from value to index.
2. For each index `i` with value `x`:
   - If `target - x` is in `seen`, return `[seen[target - x], i]`.
   - Otherwise record `seen[x] = i`.

Looking up the complement *before* inserting the current value guarantees an element is never
paired with itself, and handles duplicates such as `[3,3]` naturally.

```python
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i, x in enumerate(nums):
            if target - x in seen:
                return [seen[target - x], i]
            seen[x] = i
```

- **Time:** `O(n)` — each element is visited once with `O(1)` expected map operations.
- **Space:** `O(n)` for the map.
