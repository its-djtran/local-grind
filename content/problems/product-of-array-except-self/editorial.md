## Approach 1: Brute Force

For every index, multiply together all the other elements.

```python
class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        answer = []
        for i in range(n):
            prod = 1
            for j in range(n):
                if j != i:
                    prod *= nums[j]
            answer.append(prod)
        return answer
```

- **Time:** `O(n^2)` — hopeless for `10^5` elements.
- **Space:** `O(1)` besides the output.

A tempting shortcut is to multiply everything once and divide by `nums[i]`. Division is
forbidden here, and it also breaks down when the array contains zeros.

## Approach 2: Left and Right Product Arrays

**Intuition.** Split the product for index `i` into two pieces: everything strictly to its left
and everything strictly to its right. Both pieces can be precomputed for all indices with one
pass each.

**Algorithm.**

1. `left[0] = 1`, and `left[i] = left[i - 1] * nums[i - 1]`.
2. `right[n - 1] = 1`, and `right[i] = right[i + 1] * nums[i + 1]`.
3. `answer[i] = left[i] * right[i]`.

```python
class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        left, right = [1] * n, [1] * n
        for i in range(1, n):
            left[i] = left[i - 1] * nums[i - 1]
        for i in range(n - 2, -1, -1):
            right[i] = right[i + 1] * nums[i + 1]
        return [left[i] * right[i] for i in range(n)]
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the two helper arrays.

## Approach 3: Output Array Plus a Running Suffix (Optimal)

**Intuition.** The `right` array is only ever read in reverse order, one entry at a time, so a
single variable can stand in for it. Store the left products straight into `answer`, then walk
backwards multiplying each slot by the product of everything seen so far on the right.

**Algorithm.**

1. Fill `answer[i]` with the product of `nums[0..i-1]` in a forward pass.
2. Keep `suffix = 1`. Going from the last index to the first, set `answer[i] *= suffix`, then
   `suffix *= nums[i]`.

Zeros need no special treatment: they simply propagate through the prefix and suffix products.

```python
class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        answer = [1] * n
        prefix = 1
        for i in range(n):
            answer[i] = prefix
            prefix *= nums[i]
        suffix = 1
        for i in range(n - 1, -1, -1):
            answer[i] *= suffix
            suffix *= nums[i]
        return answer
```

- **Time:** `O(n)` — two passes.
- **Space:** `O(1)` extra beyond the output array.
