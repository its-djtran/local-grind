## Approach 1: Plain Recursion

**Intuition.** The final move onto step `n` is either a single step from `n - 1` or a double step
from `n - 2`. Every way of reaching the top therefore extends exactly one way of reaching one of
those two steps, so `ways(n) = ways(n - 1) + ways(n - 2)` with `ways(1) = 1` and `ways(2) = 2`.

**Algorithm.** Translate the recurrence directly into a recursive function.

```python
class Solution:
    def climbStairs(self, n: int) -> int:
        if n <= 2:
            return n
        return self.climbStairs(n - 1) + self.climbStairs(n - 2)
```

- **Time:** `O(2^n)` — the call tree branches twice at almost every level and recomputes the same
  subproblems many times; `n = 45` takes billions of calls.
- **Space:** `O(n)` for the recursion depth.

## Approach 2: Memoized Recursion

**Intuition.** There are only `n` distinct subproblems. Remembering each answer the first time it
is computed collapses the exponential call tree into a straight line.

**Algorithm.** Wrap the recursive function with a cache.

```python
class Solution:
    def climbStairs(self, n: int) -> int:
        @lru_cache(maxsize=None)
        def ways(k: int) -> int:
            if k <= 2:
                return k
            return ways(k - 1) + ways(k - 2)

        return ways(n)
```

- **Time:** `O(n)` — each value is computed once.
- **Space:** `O(n)` for the cache and the call stack.

## Approach 3: Bottom-Up Table

**Intuition.** Instead of starting at `n` and recursing down, fill the answers in increasing order
so that both values each step depends on are already known.

**Algorithm.**

1. Create `dp` of size `n + 1` with `dp[0] = 1` (one way to stand at the bottom) and `dp[1] = 1`.
2. For `k` from `2` to `n`, set `dp[k] = dp[k - 1] + dp[k - 2]`.
3. Return `dp[n]`.

```python
class Solution:
    def climbStairs(self, n: int) -> int:
        dp = [1] * (n + 1)
        for k in range(2, n + 1):
            dp[k] = dp[k - 1] + dp[k - 2]
        return dp[n]
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the table.

## Approach 4: Two Rolling Variables (Optimal)

**Intuition.** Each entry of the table only looks two entries back, so the rest of the table is
dead weight. Keep just the last two values. (These are the Fibonacci numbers shifted by one.)

**Algorithm.**

1. Let `prev = 1` (ways to reach step 0) and `cur = 1` (ways to reach step 1).
2. Repeat `n - 1` times: `prev, cur = cur, prev + cur`.
3. Return `cur`.

```python
class Solution:
    def climbStairs(self, n: int) -> int:
        prev, cur = 1, 1
        for _ in range(n - 1):
            prev, cur = cur, prev + cur
        return cur
```

- **Time:** `O(n)`.
- **Space:** `O(1)`.

A closed form (Binet's formula) or `2 x 2` matrix exponentiation can reach `O(log n)`, but for
`n <= 45` the linear loop is already instantaneous.
