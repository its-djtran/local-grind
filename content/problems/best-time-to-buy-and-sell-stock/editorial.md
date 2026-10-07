## Approach 1: Brute Force

Try every buy day `i` together with every later sell day `j` and keep the best difference.

```python
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        best = 0
        for i in range(len(prices)):
            for j in range(i + 1, len(prices)):
                best = max(best, prices[j] - prices[i])
        return best
```

- **Time:** `O(n^2)` — every pair of days is examined; far too slow for `10^5` days.
- **Space:** `O(1)`.

## Approach 2: Track the Running Minimum (Optimal)

**Intuition.** Fix the sell day `j`. The best purchase for that sale is simply the cheapest price
on any day before `j`. That minimum can be maintained incrementally as we move `j` forward, so
we never need to look back.

**Algorithm.**

1. Set `low = prices[0]` and `best = 0`.
2. For each later price `p`:
   - Update `best = max(best, p - low)` (sell today after buying at the cheapest earlier day).
   - Update `low = min(low, p)` so tomorrow can buy at today's price.
3. Return `best`.

Because `best` starts at `0`, a strictly falling market returns `0` without special handling.
This is also a tiny dynamic program: `low` is the best "holding" state and `best` the best
"sold" state after each day.

```python
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        low, best = prices[0], 0
        for p in prices:
            if p < low:
                low = p
            elif p - low > best:
                best = p - low
        return best
```

- **Time:** `O(n)` — a single pass.
- **Space:** `O(1)`.
