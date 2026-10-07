## Approach 1: Brute Force Counting

**Intuition.** For each element, count how many times it occurs; return the first one whose count
exceeds `n // 2`.

```python
class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        half = len(nums) // 2
        for x in nums:
            count = 0
            for y in nums:
                if y == x:
                    count += 1
            if count > half:
                return x
```

- **Time:** `O(n^2)`.
- **Space:** `O(1)`.

## Approach 2: Hash Map

**Intuition.** Count every value in a single pass, then pick the value with a count above `n // 2`.

```python
class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        counts = Counter(nums)
        return max(counts, key=counts.get)
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the counts.

## Approach 3: Sorting

**Intuition.** In sorted order the majority element forms one contiguous block longer than half
the array. Any such block must cover the middle position `n // 2`, whether it starts at the very
beginning or ends at the very end.

```python
class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        return sorted(nums)[len(nums) // 2]
```

- **Time:** `O(n log n)`.
- **Space:** `O(n)` for the sorted copy (`O(1)` extra when sorting in place).

## Approach 4: Boyer-Moore Voting (Optimal)

**Intuition.** Imagine every element as a vote. Whenever two *different* values meet, they cancel
each other out. Each cancellation removes at most one copy of the majority element along with one
other element, and the majority element has more copies than all other values combined, so it
cannot be fully cancelled: whatever survives at the end must be the majority.

The algorithm implements this with a single candidate and a counter. The counter tracks how many
uncancelled copies of the candidate remain in the prefix seen so far; when it drops to zero, that
prefix has cancelled itself out completely, and the majority element is still a majority of the
remaining suffix.

**Algorithm.**

1. Set `candidate = 0`, `count = 0`.
2. For each `x` in `nums`:
   - If `count == 0`, set `candidate = x`.
   - Increment `count` if `x == candidate`, otherwise decrement it.
3. Return `candidate`.

The final verification pass that Boyer-Moore normally needs is unnecessary here because a
majority element is guaranteed to exist.

```python
class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        candidate, count = 0, 0
        for x in nums:
            if count == 0:
                candidate = x
            count += 1 if x == candidate else -1
        return candidate
```

- **Time:** `O(n)` — one pass.
- **Space:** `O(1)`.
