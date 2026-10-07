## Approach 1: Counting Sort (Two Passes)

**Intuition.** With only three possible values, the sorted array is fully determined by how many
`0`s, `1`s and `2`s there are. Count them, then overwrite the array.

```python
class Solution:
    def sortColors(self, nums: List[int]) -> None:
        counts = [0, 0, 0]
        for x in nums:
            counts[x] += 1
        i = 0
        for color in range(3):
            for _ in range(counts[color]):
                nums[i] = color
                i += 1
```

- **Time:** `O(n)` — one pass to count, one to write.
- **Space:** `O(1)`.

## Approach 2: Dutch National Flag (Optimal)

**Intuition.** Maintain four regions in the array as we sweep it once:

- `nums[0 .. low-1]` are all `0`,
- `nums[low .. mid-1]` are all `1`,
- `nums[mid .. high]` are not yet examined,
- `nums[high+1 .. n-1]` are all `2`.

Each step examines `nums[mid]` and shrinks the unexamined region by one.

**Algorithm.**

1. `low = mid = 0`, `high = n - 1`.
2. While `mid <= high`:
   - `0`: swap it with `nums[low]` (which is a `1`, or `mid` itself), then advance both `low`
     and `mid`.
   - `1`: it is already in the right region; advance `mid`.
   - `2`: swap it with `nums[high]` and decrease `high`. Do **not** advance `mid`: the value just
     brought in from the right has not been examined.

```python
class Solution:
    def sortColors(self, nums: List[int]) -> None:
        low, mid, high = 0, 0, len(nums) - 1
        while mid <= high:
            x = nums[mid]
            if x == 0:
                nums[low], nums[mid] = nums[mid], nums[low]
                low += 1
                mid += 1
            elif x == 1:
                mid += 1
            else:
                nums[mid], nums[high] = nums[high], nums[mid]
                high -= 1
```

- **Time:** `O(n)` — a single pass; every step either advances `mid` or shrinks `high`.
- **Space:** `O(1)`.
