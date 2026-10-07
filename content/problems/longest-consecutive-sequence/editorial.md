## Approach 1: Walk Upward From Every Number

Put the values in a set, then from every value count how far the run continues upward.

```python
class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        present = set(nums)
        best = 0
        for x in present:
            length = 1
            while x + length in present:
                length += 1
            best = max(best, length)
        return best
```

- **Time:** `O(n^2)` in the worst case: for a single run of `n` numbers the walks have lengths
  `n, n - 1, ..., 1`.
- **Space:** `O(n)`.

## Approach 2: Sort

**Intuition.** In sorted order a consecutive run occupies a contiguous block (with possible
repeats). Scan once, extending the current run when the next distinct value is exactly one larger
and restarting otherwise.

```python
class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        if not nums:
            return 0
        s = sorted(set(nums))
        best = cur = 1
        for a, b in zip(s, s[1:]):
            cur = cur + 1 if b == a + 1 else 1
            best = max(best, cur)
        return best
```

- **Time:** `O(n log n)`.
- **Space:** `O(n)`.

## Approach 3: Hash Set, Counting Only From Run Starts (Optimal)

**Intuition.** Approach 1 wastes time re-walking the same run from each of its members. A run
has exactly one starting value: the `x` for which `x - 1` is absent. If we walk upward only from
starting values, every number is stepped over by exactly one walk.

**Algorithm.**

1. Build `present = set(nums)`.
2. For each `x` in `present` with `x - 1` not in `present`, walk `x, x + 1, ...` while the values
   are present and record the length.
3. Return the longest length (`0` when the array is empty).

Iterating over the set rather than the original array also avoids repeating work for duplicate
starting values.

```python
class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        present = set(nums)
        best = 0
        for x in present:
            if x - 1 not in present:
                y = x + 1
                while y in present:
                    y += 1
                if y - x > best:
                    best = y - x
        return best
```

- **Time:** `O(n)` expected — each value is checked a constant number of times.
- **Space:** `O(n)` for the set.
