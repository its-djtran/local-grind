## Approach 1: Brute-Force Recursion

**Intuition.** Make the usual rob-or-skip decision house by house, but also remember whether the
first house was robbed. If it was, the last house is forbidden.

```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1:
            return nums[0]

        def best(i: int, took_first: bool) -> int:
            if i >= n or (i == n - 1 and took_first):
                return 0
            return max(best(i + 1, took_first), nums[i] + best(i + 2, took_first))

        return max(nums[0] + best(2, True), best(1, False))
```

- **Time:** `O(phi^n)` (about `1.618^n`) — exponential, hopeless for `n = 100`.
- **Space:** `O(n)` for the recursion.

## Approach 2: Memoized Recursion

**Intuition.** The state is just `(i, took_first)`, so there are `2n` subproblems.

```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1:
            return nums[0]

        @lru_cache(maxsize=None)
        def best(i: int, took_first: bool) -> int:
            if i >= n or (i == n - 1 and took_first):
                return 0
            return max(best(i + 1, took_first), nums[i] + best(i + 2, took_first))

        return max(nums[0] + best(2, True), best(1, False))
```

- **Time:** `O(n)`.
- **Space:** `O(n)`.

## Approach 3: Break the Circle Twice (Optimal)

**Intuition.** In any valid plan, the first house or the last house (or both) is left alone.

- If the last house is left alone, the problem is the straight-line House Robber on
  `nums[0 .. n-2]`.
- If the first house is left alone, it is the straight-line problem on `nums[1 .. n-1]`.

Every valid plan falls into at least one case, and every plan found in either case is valid on
the circle, so the answer is the better of the two. A single house has no neighbor at all and is
handled separately (both slices would be empty).

**Algorithm.**

1. If `n == 1`, return `nums[0]`.
2. Define `line(lo, hi)`: the two-variable House Robber recurrence over `nums[lo:hi]`.
3. Return `max(line(0, n - 1), line(1, n))`.

```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1:
            return nums[0]

        def line(lo: int, hi: int) -> int:
            prev = cur = 0
            for i in range(lo, hi):
                prev, cur = cur, max(cur, prev + nums[i])
            return cur

        return max(line(0, n - 1), line(1, n))
```

- **Time:** `O(n)` — two linear passes.
- **Space:** `O(1)`.
