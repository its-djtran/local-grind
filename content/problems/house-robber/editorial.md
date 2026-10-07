## Approach 1: Brute-Force Recursion

**Intuition.** Standing in front of house `i`, there are two choices: skip it and move on to
house `i + 1`, or rob it and jump to house `i + 2` (the neighbor is now off-limits). The best
plan is the better of the two.

```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        def best(i: int) -> int:
            if i >= len(nums):
                return 0
            return max(best(i + 1), nums[i] + best(i + 2))

        return best(0)
```

- **Time:** `O(phi^n)` (about `1.618^n`) — the call tree grows like the Fibonacci numbers, which is
  far too slow for `n = 100`.
- **Space:** `O(n)` for the recursion.

## Approach 2: Memoized Recursion

**Intuition.** `best(i)` depends only on `i`, so there are just `n` different subproblems. Cache
them.

```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        @lru_cache(maxsize=None)
        def best(i: int) -> int:
            if i >= len(nums):
                return 0
            return max(best(i + 1), nums[i] + best(i + 2))

        return best(0)
```

- **Time:** `O(n)`.
- **Space:** `O(n)`.

## Approach 3: Bottom-Up Table

**Intuition.** Let `dp[i]` be the most money obtainable from the first `i` houses. Either house
`i - 1` is skipped (`dp[i - 1]`) or it is robbed, in which case house `i - 2` is not
(`dp[i - 2] + nums[i - 1]`).

```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [0] * (n + 1)
        dp[1] = nums[0]
        for i in range(2, n + 1):
            dp[i] = max(dp[i - 1], dp[i - 2] + nums[i - 1])
        return dp[n]
```

- **Time:** `O(n)`.
- **Space:** `O(n)`.

## Approach 4: Two Variables (Optimal)

**Intuition.** Each table entry only looks back two places. Keep `prev` (the best up to two houses
ago) and `cur` (the best up to the previous house), and slide them forward.

**Algorithm.**

1. `prev = cur = 0`.
2. For each amount `x`: `prev, cur = cur, max(cur, prev + x)`.
3. Return `cur`.

```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        prev = cur = 0
        for x in nums:
            prev, cur = cur, max(cur, prev + x)
        return cur
```

- **Time:** `O(n)`.
- **Space:** `O(1)`.
