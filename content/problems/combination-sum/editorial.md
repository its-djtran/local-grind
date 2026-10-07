## Approach 1: Ordered Sequences, Then Deduplicate (Brute Force)

**Intuition.** Generate every *sequence* of candidates that adds up to the target, then collapse
sequences that contain the same numbers by sorting each one and storing it in a set.

```python
class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        seen = set()

        def build(remaining, seq):
            if remaining == 0:
                seen.add(tuple(sorted(seq)))
                return
            for c in candidates:
                if c <= remaining:
                    seq.append(c)
                    build(remaining - c, seq)
                    seq.pop()

        build(target, [])
        return [list(t) for t in seen]
```

- **Time:** exponential in the number of *ordered* sequences, which can be vastly larger than the
  number of combinations. With candidates `2` and `3` and a target of `40`, there are thousands
  of orderings for every combination.
- **Space:** `O(target)` recursion depth plus the result set.

## Approach 2: Backtracking in Non-Decreasing Index Order (Optimal)

**Intuition.** Generate each combination in exactly one canonical order: list its numbers in the
order their candidates appear in the (sorted) array. A recursive call is then only allowed to pick
candidates at or after the index of the previous pick. Staying on the same index permits reuse,
and never going back prevents duplicates, so no deduplication is needed.

**Algorithm.**

1. Sort `candidates`.
2. `backtrack(start, remaining)`: if `remaining == 0`, record a copy of the current combination.
   Otherwise, for each index `i >= start`, stop if `candidates[i] > remaining` (sorting makes
   every later candidate too large as well); else push it, recurse with `(i, remaining -
   candidates[i])`, and pop.

```python
class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates = sorted(candidates)
        result, combo = [], []

        def backtrack(start, remaining):
            if remaining == 0:
                result.append(combo[:])
                return
            for i in range(start, len(candidates)):
                c = candidates[i]
                if c > remaining:
                    break
                combo.append(c)
                backtrack(i, remaining - c)
                combo.pop()

        backtrack(0, target)
        return result
```

- **Time:** proportional to the number of nodes in the search tree, `O(N^(T/M + 1))` as a loose
  bound for `N` candidates, target `T` and smallest candidate `M`. Every leaf that is reached is
  either a valid combination or cut off by the sorted-order break.
- **Space:** `O(T / M)` for the recursion depth and current combination, excluding the output.
