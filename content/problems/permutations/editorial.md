## Approach 1: Insert Into Every Gap

**Intuition.** Every permutation of the first `k + 1` numbers comes from a permutation of the
first `k` numbers with the new number inserted into one of its `k + 1` gaps. Grow the list of
permutations one number at a time.

```python
class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        perms = [[]]
        for x in nums:
            perms = [p[:i] + [x] + p[i:] for p in perms for i in range(len(p) + 1)]
        return perms
```

- **Time:** `O(n * n!)` — there are `n!` permutations and each costs `O(n)` to build.
- **Space:** `O(n * n!)` for the output.

## Approach 2: Backtracking with a Used Set (Optimal)

**Intuition.** Fill the permutation from left to right. At each position, try every number that
has not been placed yet; after exploring that choice, remove it again so the next choice starts
from the same state. When the permutation has `n` numbers, record a copy.

**Algorithm.**

1. Keep `path` (the partial permutation) and `used` (a boolean per index).
2. If `len(path) == n`, append `path[:]` to the result.
3. Otherwise, for each unused index `i`: mark it, append `nums[i]`, recurse, then pop and unmark.

```python
class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        result, path, used = [], [], [False] * len(nums)

        def backtrack():
            if len(path) == len(nums):
                result.append(path[:])
                return
            for i, x in enumerate(nums):
                if not used[i]:
                    used[i] = True
                    path.append(x)
                    backtrack()
                    path.pop()
                    used[i] = False

        backtrack()
        return result
```

Every recorded list is a distinct ordering because two branches differ in the first position
where they chose different numbers.

- **Time:** `O(n * n!)` — `n!` leaves, each copied in `O(n)`; the internal nodes add at most a
  constant factor. No algorithm can do better, since the output itself has `n * n!` numbers.
- **Space:** `O(n)` for the recursion, `path` and `used`, excluding the output.
