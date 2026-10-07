## Approach 1: Brute-Force Recursion

**Intuition.** Walk through the array deciding, for each element, whether to keep it. An element
can be kept only if it is larger than the last element kept. Trying both choices everywhere
explores every increasing subsequence.

```python
class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        def longest(i: int, last: float) -> int:
            if i == len(nums):
                return 0
            skip = longest(i + 1, last)
            if nums[i] > last:
                return max(skip, 1 + longest(i + 1, nums[i]))
            return skip

        return longest(0, -math.inf)
```

- **Time:** `O(2^n)` — every subset of positions may be explored.
- **Space:** `O(n)` for the recursion.

## Approach 2: Dynamic Programming over End Positions

**Intuition.** Let `dp[i]` be the length of the longest increasing subsequence that **ends at**
index `i`. Such a subsequence is either `nums[i]` alone, or a subsequence ending at some earlier
`j` with `nums[j] < nums[i]`, extended by `nums[i]`.

**Algorithm.**

1. Initialize `dp[i] = 1` for every `i`.
2. For each `i`, for each `j < i` with `nums[j] < nums[i]`: `dp[i] = max(dp[i], dp[j] + 1)`.
3. The answer is `max(dp)`.

```python
class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [1] * n
        for i in range(n):
            for j in range(i):
                if nums[j] < nums[i] and dp[j] + 1 > dp[i]:
                    dp[i] = dp[j] + 1
        return max(dp)
```

- **Time:** `O(n^2)` — about three million pair checks for `n = 2500`.
- **Space:** `O(n)`.

## Approach 3: Patience Sorting with Binary Search (Optimal)

**Intuition.** Maintain `tails`, where `tails[L - 1]` is the **smallest possible last value** of
any increasing subsequence of length `L` seen so far. A smaller tail is always at least as useful
as a larger one, because it leaves more room for future elements. `tails` is strictly increasing,
so it can be binary-searched.

When a new value `x` arrives:

- If `x` is larger than every tail, it extends the longest subsequence: append it.
- Otherwise find the first tail `>= x` and replace it with `x`. A subsequence of that length can
  now end in a smaller value. (Using "`>=`" rather than "`>`" keeps the subsequence strict, so
  duplicates never extend it.)

The length of `tails` at the end is the answer. Note that `tails` itself is generally *not* a
valid subsequence; only its length is meaningful.

**Algorithm.**

1. Start with an empty list `tails`.
2. For each `x` in `nums`, let `k = bisect_left(tails, x)`. If `k == len(tails)`, append `x`;
   otherwise set `tails[k] = x`.
3. Return `len(tails)`.

```python
class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        tails = []
        for x in nums:
            k = bisect_left(tails, x)
            if k == len(tails):
                tails.append(x)
            else:
                tails[k] = x
        return len(tails)
```

- **Time:** `O(n log n)` — one binary search per element.
- **Space:** `O(n)` for `tails`.
