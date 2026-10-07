## Approach 1: Brute Force

Try every triple of positions, and store each zero-sum triple in sorted form inside a set so that
repeats collapse.

```python
class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        found = set()
        for i in range(n):
            for j in range(i + 1, n):
                for k in range(j + 1, n):
                    if nums[i] + nums[j] + nums[k] == 0:
                        found.add(tuple(sorted((nums[i], nums[j], nums[k]))))
        return [list(t) for t in found]
```

- **Time:** `O(n^3)` — about `4.5 * 10^9` triples for `n = 3000`.
- **Space:** `O(m)` for the `m` distinct triplets.

## Approach 2: Fix One Value, Hash the Rest

**Intuition.** Fixing `nums[i]` turns the task into Two Sum with target `-nums[i]` on the
elements after `i`, which a hash set solves in linear time. Sorting the triplet before adding it to
a result set removes repeats.

```python
class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        found = set()
        for i in range(n):
            seen = set()
            for j in range(i + 1, n):
                need = -nums[i] - nums[j]
                if need in seen:
                    found.add(tuple(sorted((nums[i], nums[j], need))))
                seen.add(nums[j])
        return [list(t) for t in found]
```

- **Time:** `O(n^2)` expected.
- **Space:** `O(n + m)` for the helper set and the results.

## Approach 3: Sort and Two Pointers (Optimal)

**Intuition.** After sorting, fix the smallest value `a = nums[i]` and look for a pair in
`nums[i+1:]` summing to `-a`. In a sorted range the pair can be found with two pointers: if the
current sum is too small, move the left pointer right; if too large, move the right pointer left.
Sorting also makes duplicates adjacent, so they can be skipped instead of filtered with a set.

**Algorithm.**

1. Sort `nums`.
2. For each index `i`:
   - If `nums[i] > 0`, stop — three positive numbers cannot sum to zero.
   - If `nums[i]` equals `nums[i - 1]`, skip it; that first value was already handled.
   - Set `lo = i + 1`, `hi = n - 1`. While `lo < hi`, compare `s = nums[i] + nums[lo] + nums[hi]`
     with `0` and move the matching pointer. On a hit, record the triplet, move both pointers
     inward, and advance `lo` past values equal to the one just used.
3. Return the recorded triplets.

```python
class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        n = len(nums)
        res = []
        for i in range(n - 2):
            a = nums[i]
            if a > 0:
                break
            if i and a == nums[i - 1]:
                continue
            lo, hi = i + 1, n - 1
            while lo < hi:
                s = a + nums[lo] + nums[hi]
                if s < 0:
                    lo += 1
                elif s > 0:
                    hi -= 1
                else:
                    res.append([a, nums[lo], nums[hi]])
                    lo += 1
                    hi -= 1
                    while lo < hi and nums[lo] == nums[lo - 1]:
                        lo += 1
        return res
```

- **Time:** `O(n^2)` — `O(n log n)` to sort plus a linear two-pointer sweep for each `i`.
- **Space:** `O(1)` extra besides the output (ignoring the sort's internal memory).
