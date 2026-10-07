## Approach 1: Backtracking

**Intuition.** From the current index, try every jump length that is allowed and see whether any
of them eventually reaches the end.

```python
class Solution:
    def canJump(self, nums: List[int]) -> bool:
        last = len(nums) - 1

        def reach(i: int) -> bool:
            if i == last:
                return True
            for j in range(min(last, i + nums[i]), i, -1):
                if reach(j):
                    return True
            return False

        return reach(0)
```

- **Time:** `O(2^n)` in the worst case — the same index is re-explored along every route leading to
  it.
- **Space:** `O(n)` for the recursion.

## Approach 2: Dynamic Programming over Indices

**Intuition.** Whether index `i` can reach the end does not depend on how we got to `i`. Working
from right to left, mark index `i` as *good* if some index within its jump range is already good.

```python
class Solution:
    def canJump(self, nums: List[int]) -> bool:
        n = len(nums)
        good = [False] * n
        good[-1] = True
        for i in range(n - 2, -1, -1):
            for j in range(i + 1, min(n - 1, i + nums[i]) + 1):
                if good[j]:
                    good[i] = True
                    break
        return good[0]
```

- **Time:** `O(n^2)` in the worst case. When nothing ahead is good (for example
  `[n-2, n-3, ..., 1, 0, x]`, where every index can reach the `0` but no further), each index scans
  its whole range — about `5 * 10^7` checks for `n = 10^4`.
- **Space:** `O(n)`.

## Approach 3: Greedy Farthest Reach (Optimal)

**Intuition.** If index `i` is reachable, every index before it is reachable too (you can always
make a shorter jump). So the reachable indices always form a prefix `0 .. far`, and a single number
`far` describes them completely.

**Algorithm.**

1. `far = 0`.
2. For each index `i` in order: if `i > far`, index `i` is unreachable, so return `false`.
   Otherwise update `far = max(far, i + nums[i])`, and return `true` as soon as `far` reaches the
   last index.
3. If the loop finishes, return `true`.

```python
class Solution:
    def canJump(self, nums: List[int]) -> bool:
        far, last = 0, len(nums) - 1
        for i, step in enumerate(nums):
            if i > far:
                return False
            if i + step > far:
                far = i + step
                if far >= last:
                    return True
        return True
```

- **Time:** `O(n)` — one pass.
- **Space:** `O(1)`.

An equivalent greedy scans from right to left, keeping the leftmost index known to reach the end
and moving it to `i` whenever `i + nums[i]` reaches it.
