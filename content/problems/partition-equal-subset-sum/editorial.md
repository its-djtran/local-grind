## Approach 1: Brute-Force Recursion

**Intuition.** Two equal groups each sum to `total / 2`, so the total must be even, and then the
question is whether some subset reaches exactly `half = total / 2` (the remaining elements
automatically form the other group). Decide for each element whether it joins the subset.

```python
class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2:
            return False

        def pick(i: int, need: int) -> bool:
            if need == 0:
                return True
            if i == len(nums) or need < 0:
                return False
            return pick(i + 1, need - nums[i]) or pick(i + 1, need)

        return pick(0, total // 2)
```

- **Time:** `O(2^n)` when no subset works but the search cannot rule it out early.
- **Space:** `O(n)` for the recursion.

## Approach 2: Memoized Recursion

**Intuition.** The state is just `(i, need)`, with `need` never exceeding `half`. Caching gives at
most `n * half` distinct states.

```python
class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2:
            return False

        @lru_cache(maxsize=None)
        def pick(i: int, need: int) -> bool:
            if need == 0:
                return True
            if i == len(nums) or need < 0:
                return False
            return pick(i + 1, need - nums[i]) or pick(i + 1, need)

        return pick(0, total // 2)
```

- **Time:** `O(n * half)` — up to `200 * 10^4` states.
- **Space:** `O(n * half)` for the cache, which is heavy in Python.

## Approach 3: Bottom-Up Subset Sum

**Intuition.** Let `can[s]` say whether some subset of the elements processed so far sums to `s`.
Initially only `can[0]` is true. Processing an element `x` makes every `s` with `can[s - x]` true
reachable as well. Sweeping `s` from high to low ensures each element is used at most once
(lower entries still hold the values from before `x` was considered).

```python
class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2:
            return False
        half = total // 2
        can = [True] + [False] * half
        for x in nums:
            for s in range(half, x - 1, -1):
                if can[s - x]:
                    can[s] = True
            if can[half]:
                return True
        return can[half]
```

- **Time:** `O(n * half)`.
- **Space:** `O(half)`.

## Approach 4: Bitset of Reachable Sums (Optimal)

**Intuition.** The boolean array from Approach 3 is just a set of bits. Store it in one Python
integer where bit `s` is set when sum `s` is reachable. Adding element `x` is then a single
operation: `bits |= bits << x` (each old sum `s` also produces `s + x`). The shift and OR run in
optimized native code over machine words, which is dramatically faster than a Python loop.

**Algorithm.**

1. If the total is odd, return `false`.
2. Start with `bits = 1` (only sum `0`). For each `x`: `bits |= bits << x`.
3. Return whether bit `half` is set.

```python
class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2:
            return False
        half = total // 2
        bits = 1
        for x in nums:
            bits |= bits << x
        return (bits >> half) & 1 == 1
```

- **Time:** `O(n * total / w)` where `w` is the machine word size — the same asymptotic work as
  Approach 3 divided by roughly 64.
- **Space:** `O(total / w)` for the integer.
