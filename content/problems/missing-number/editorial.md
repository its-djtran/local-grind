## Approach 1: Check Every Candidate

**Intuition.** Try each value `0, 1, ..., n` and return the first one that does not appear in the
array.

```python
class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        for v in range(len(nums) + 1):
            if v not in nums:
                return v
```

- **Time:** `O(n^2)` — each membership test scans the whole list.
- **Space:** `O(1)`.

## Approach 2: Sorting

**Intuition.** After sorting, a complete range would satisfy `nums[i] == i` everywhere. The first
index where that fails is the missing value; if none fails, the missing value is `n`.

```python
class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        nums = sorted(nums)
        for i, x in enumerate(nums):
            if x != i:
                return i
        return len(nums)
```

- **Time:** `O(n log n)` for the sort.
- **Space:** `O(n)` for the sorted copy (`O(1)` extra if you sort in place and may modify the input).

## Approach 3: Hash Set

**Intuition.** Put every value in a set so each membership test is `O(1)`, then scan the range.

```python
class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        present = set(nums)
        for v in range(len(nums) + 1):
            if v not in present:
                return v
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the set.

## Approach 4: Gauss's Formula

**Intuition.** The numbers `0..n` add up to `n * (n + 1) / 2`. Every one of them except the missing
value is in the array, so subtracting the array's sum from that total leaves exactly the missing
value.

```python
class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        n = len(nums)
        return n * (n + 1) // 2 - sum(nums)
```

- **Time:** `O(n)`.
- **Space:** `O(1)`.

## Approach 5: XOR (Optimal)

**Intuition.** XOR is its own inverse: `x ^ x == 0` and `x ^ 0 == x`, and the order of operations
does not matter. If we XOR together every index `0..n` and every value in the array, each number
that appears in both lists cancels itself out, and only the missing value survives. Unlike the sum
approach, intermediate results never grow beyond the bit width of `n`, which matters in languages
with fixed-size integers.

**Algorithm.**

1. Start with `missing = n` (the one index that `enumerate` does not produce).
2. For each index `i` with value `x`, update `missing ^= i ^ x`.
3. Return `missing`.

```python
class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        missing = len(nums)
        for i, x in enumerate(nums):
            missing ^= i ^ x
        return missing
```

- **Time:** `O(n)` — a single pass.
- **Space:** `O(1)`.
