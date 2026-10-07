## Approach 1: Bitmasks

**Intuition.** Number the elements `0` to `n - 1`. A subset is fully described by which elements
it contains, which is exactly an `n`-bit number: bit `i` is set when `nums[i]` is included. So the
integers `0` through `2^n - 1` enumerate every subset exactly once.

```python
class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        return [[nums[i] for i in range(n) if mask >> i & 1] for mask in range(1 << n)]
```

- **Time:** `O(n * 2^n)` — `2^n` masks, each scanned over `n` bits.
- **Space:** `O(n * 2^n)` for the output.

## Approach 2: Backtracking

**Intuition.** Decide element by element. A recursive call that starts at index `start` records
the current subset, then tries extending it with each later element in turn. Only moving forward
guarantees each subset is produced once, in increasing index order.

```python
class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        result, path = [], []

        def backtrack(start):
            result.append(path[:])
            for i in range(start, len(nums)):
                path.append(nums[i])
                backtrack(i + 1)
                path.pop()

        backtrack(0)
        return result
```

- **Time:** `O(n * 2^n)`: one call per subset, each copying up to `n` numbers.
- **Space:** `O(n)` recursion depth, excluding the output.

## Approach 3: Iterative Doubling (Optimal)

**Intuition.** The subsets of the first `k + 1` elements are the subsets of the first `k`
elements, plus each of those same subsets with element `k + 1` added. Start from `[[]]` and double
the list once per element.

**Algorithm.**

1. `result = [[]]`.
2. For each `x` in `nums`, append `s + [x]` for every subset `s` currently in `result`.

```python
class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        result = [[]]
        for x in nums:
            result += [s + [x] for s in result]
        return result
```

- **Time:** `O(n * 2^n)`, which matches the size of the output, so no approach can be
  asymptotically faster.
- **Space:** `O(n * 2^n)` for the output; no extra space beyond it.
