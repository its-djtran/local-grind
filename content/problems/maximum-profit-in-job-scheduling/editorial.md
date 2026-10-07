## Approach 1: Try Every Subset

**Intuition.** Enumerate each job as either taken or skipped, prune any choice that overlaps a
job already taken, and keep the best total. Processing jobs in start order means a taken job
only has to be compared with the end of the previous taken job.

```python
class Solution:
    def jobScheduling(self, startTime: List[int], endTime: List[int], profit: List[int]) -> int:
        jobs = sorted(zip(startTime, endTime, profit))

        def best(i, free_at):
            if i == len(jobs):
                return 0
            s, e, p = jobs[i]
            skip = best(i + 1, free_at)
            take = p + best(i + 1, e) if s >= free_at else 0
            return max(skip, take)

        return best(0, 0)
```

- **Time:** `O(2^n)`.
- **Space:** `O(n)` recursion depth.

## Approach 2: Quadratic Dynamic Programming

**Intuition.** Sort jobs by end time and let `dp[i]` be the best profit using only the first
`i` jobs. Job `i - 1` is either skipped (`dp[i - 1]`) or taken, in which case the remaining jobs
must all finish by its start. Scanning backwards for the last such job costs `O(n)`.

```python
class Solution:
    def jobScheduling(self, startTime: List[int], endTime: List[int], profit: List[int]) -> int:
        jobs = sorted(zip(endTime, startTime, profit))
        n = len(jobs)
        dp = [0] * (n + 1)
        for i in range(1, n + 1):
            e, s, p = jobs[i - 1]
            j = i - 1
            while j > 0 and jobs[j - 1][0] > s:
                j -= 1
            dp[i] = max(dp[i - 1], dp[j] + p)
        return dp[n]
```

- **Time:** `O(n^2)` in the worst case (for example when many jobs overlap each other).
- **Space:** `O(n)`.

## Approach 3: Dynamic Programming with Binary Search (Optimal)

**Intuition.** The backwards scan in Approach 2 looks for the number of jobs whose end time is
`<= s`. Since the jobs are sorted by end time, that count is exactly
`bisect_right(ends, s)`, which takes `O(log n)`.

**Algorithm.**

1. Sort jobs by end time and extract the sorted `ends` list.
2. `dp[0] = 0`. For `i` from `1` to `n`, with job `(e, s, p) = jobs[i - 1]`:
   `j = bisect_right(ends, s)` and `dp[i] = max(dp[i - 1], dp[j] + p)`.
3. Return `dp[n]`.

`bisect_right` (rather than `bisect_left`) counts jobs ending *exactly* at `s` as compatible.
And `j <= i - 1` always holds because job `i - 1` ends after it starts, so `dp[j]` is already
computed.

```python
class Solution:
    def jobScheduling(self, startTime: List[int], endTime: List[int], profit: List[int]) -> int:
        jobs = sorted(zip(endTime, startTime, profit))
        ends = [e for e, _, _ in jobs]
        dp = [0] * (len(jobs) + 1)
        for i, (e, s, p) in enumerate(jobs, 1):
            j = bisect_right(ends, s)
            dp[i] = max(dp[i - 1], dp[j] + p)
        return dp[-1]
```

- **Time:** `O(n log n)` for the sort plus `n` binary searches.
- **Space:** `O(n)`.
