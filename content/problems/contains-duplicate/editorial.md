## Approach 1: Compare Every Pair

Check each pair `(i, j)` with `i < j`; if any two values match, there is a duplicate.

```python
class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                if nums[i] == nums[j]:
                    return True
        return False
```

- **Time:** `O(n^2)` — too slow for `10^5` distinct values, where no early exit happens.
- **Space:** `O(1)`.

## Approach 2: Sort, Then Check Neighbours

**Intuition.** Sorting places equal values side by side, so a duplicate exists exactly when two
adjacent elements of the sorted array are equal.

```python
class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        s = sorted(nums)
        return any(s[i] == s[i + 1] for i in range(len(s) - 1))
```

- **Time:** `O(n log n)` for the sort.
- **Space:** `O(n)` for the sorted copy (`O(1)` extra if you are allowed to sort `nums` in place).

## Approach 3: Hash Set (Optimal)

**Intuition.** Remember every value seen so far in a hash set. The first time the current value
is already in the set, we have found a repeat and can stop immediately.

**Algorithm.**

1. Start with an empty set `seen`.
2. For each `x` in `nums`: if `x` is in `seen`, return `true`; otherwise add it.
3. If the loop finishes, every value was new — return `false`.

```python
class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        seen = set()
        for x in nums:
            if x in seen:
                return True
            seen.add(x)
        return False
```

The one-liner `len(set(nums)) < len(nums)` has the same complexity, but it always builds the full
set instead of stopping at the first repeat.

- **Time:** `O(n)` expected.
- **Space:** `O(n)` for the set.
