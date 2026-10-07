## Approach 1: Brute-Force Recursion

**Intuition.** Every valid sequence has a first number `x`, and what follows it is a valid
sequence for `target - x`. Summing over every possible first number counts each sequence exactly
once. (Because order matters, there is no need to avoid "reusing" earlier choices.)

```python
class Solution:
    def combinationSum4(self, nums: List[int], target: int) -> int:
        def count(t: int) -> int:
            if t == 0:
                return 1
            return sum(count(t - x) for x in nums if x <= t)

        return count(target)
```

- **Time:** at least proportional to the answer itself, which can be over two billion, since each
  counted sequence is a separate leaf of the call tree.
- **Space:** `O(target)` for the recursion depth.

## Approach 2: Memoized Recursion

**Intuition.** `count(t)` depends only on `t`, so there are at most `target + 1` distinct calls.

```python
class Solution:
    def combinationSum4(self, nums: List[int], target: int) -> int:
        @lru_cache(maxsize=None)
        def count(t: int) -> int:
            if t == 0:
                return 1
            return sum(count(t - x) for x in nums if x <= t)

        return count(target)
```

- **Time:** `O(n * target)`.
- **Space:** `O(target)`.

## Approach 3: Bottom-Up DP (Optimal)

**Intuition.** Let `dp[t]` be the number of ordered sequences summing to `t`, with `dp[0] = 1`
for the empty sequence. Classifying sequences for `t` by their **last** number `x` gives
`dp[t] = sum(dp[t - x])` over `x <= t`. Computing totals in increasing order makes every term
available when needed.

The loop order is the whole difference between this problem and counting unordered coin
combinations: the totals loop is on the **outside** and the numbers loop on the inside, so each
`dp[t]` considers every possible last number. Swapping the loops would count each multiset once.

**Algorithm.**

1. Sort `nums` so the inner loop can stop as soon as a number exceeds `t`.
2. `dp = [1] + [0] * target`.
3. For `t` from `1` to `target`, for each `x` in `nums` with `x <= t`: `dp[t] += dp[t - x]`.
4. Return `dp[target]`.

```python
class Solution:
    def combinationSum4(self, nums: List[int], target: int) -> int:
        nums = sorted(nums)
        dp = [1] + [0] * target
        for t in range(1, target + 1):
            total = 0
            for x in nums:
                if x > t:
                    break
                total += dp[t - x]
            dp[t] = total
        return dp[target]
```

- **Time:** `O(n log n + n * target)` — at most `200 * 1000` additions.
- **Space:** `O(target)`.

Intermediate entries `dp[t]` for `t < target` may exceed 32 bits even when the final answer does
not. Python integers grow as needed; in fixed-width languages you would cap or skip overflowing
values.

**Follow-up.** With negative numbers, a pair such as `1` and `-1` can be repeated forever without
changing the sum, so infinitely many sequences may exist. The problem would need an extra limit,
for example a maximum sequence length, which would then become a second DP dimension.
