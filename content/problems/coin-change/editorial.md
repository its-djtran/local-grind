## Approach 1: Brute-Force Recursion

**Intuition.** Whatever the optimal set of coins is, some coin `c` is used "last". Removing it
leaves an optimal way to make `amount - c`. So the answer for `x` is one more than the best answer
among `x - c` for every coin `c <= x`. A greedy choice (always take the largest coin) is tempting
but wrong: for coins `[1, 3, 4]` and amount `6`, greedy picks `4 + 1 + 1` while `3 + 3` is better.

**Algorithm.** Recurse on every possible last coin and take the minimum, returning infinity for
negative amounts.

```python
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        def fewest(x: int) -> float:
            if x == 0:
                return 0
            best = math.inf
            for c in coins:
                if c <= x:
                    best = min(best, fewest(x - c) + 1)
            return best

        ans = fewest(amount)
        return -1 if ans == math.inf else ans
```

- **Time:** `O(k^(amount / m))` where `k` is the number of coin types and `m` the smallest coin —
  the same remainders are solved again and again.
- **Space:** `O(amount / m)` for the recursion depth.

## Approach 2: Memoized Recursion

**Intuition.** There are only `amount + 1` different remainders. Caching the answer for each one
means each is solved once.

```python
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        @lru_cache(maxsize=None)
        def fewest(x: int) -> float:
            if x == 0:
                return 0
            best = math.inf
            for c in coins:
                if c <= x:
                    best = min(best, fewest(x - c) + 1)
            return best

        ans = fewest(amount)
        return -1 if ans == math.inf else ans
```

- **Time:** `O(k * amount)`.
- **Space:** `O(amount)` for the cache. The recursion can be up to `amount` calls deep, which is
  close to Python's default recursion limit; the bottom-up version below avoids that entirely.

## Approach 3: Breadth-First Search

**Intuition.** Picture every amount from `0` to `amount` as a node, with an edge from `x` to
`x + c` for every coin `c`. The fewest coins is the length of the shortest path from `0` to
`amount`, and BFS finds shortest paths in unweighted graphs level by level.

**Algorithm.** Start a queue at `0`. Process one level at a time; each level corresponds to one
more coin. Return the level at which `amount` is first reached, or `-1` if the queue empties.

```python
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        if amount == 0:
            return 0
        seen = [False] * (amount + 1)
        seen[0] = True
        level, steps = [0], 0
        while level:
            steps += 1
            nxt = []
            for x in level:
                for c in coins:
                    y = x + c
                    if y == amount:
                        return steps
                    if y < amount and not seen[y]:
                        seen[y] = True
                        nxt.append(y)
            level = nxt
        return -1
```

- **Time:** `O(k * amount)` — every amount is enqueued at most once and tries every coin.
- **Space:** `O(amount)`.

## Approach 4: Bottom-Up Dynamic Programming (Optimal)

**Intuition.** Fill a table `dp[x]` = fewest coins for `x` in increasing order of `x`, so every
smaller remainder is already final when it is needed. Iterating coin by coin (the outer loop)
and sweeping amounts upward lets each coin be reused any number of times.

**Algorithm.**

1. Let `dp[0] = 0` and `dp[x] = amount + 1` (an impossible count) for every other `x`.
2. For each coin `c`, for each `x` from `c` to `amount`: `dp[x] = min(dp[x], dp[x - c] + 1)`.
3. Return `dp[amount]` if it is at most `amount`, otherwise `-1`.

```python
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        INF = amount + 1
        dp = [0] + [INF] * amount
        for c in coins:
            for x in range(c, amount + 1):
                if dp[x - c] + 1 < dp[x]:
                    dp[x] = dp[x - c] + 1
        return dp[amount] if dp[amount] != INF else -1
```

- **Time:** `O(k * amount)` — at most `12 * 10^4` updates.
- **Space:** `O(amount)` for the table.
