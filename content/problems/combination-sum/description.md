You are given an array of **distinct** integers `candidates` and an integer `target`. Return every **unique combination** of candidates whose sum is exactly `target`.

Each candidate may be used **any number of times** in a combination. Two combinations are considered the same if every number appears in them the same number of times, so `[2,2,3]` and `[3,2,2]` count as one combination.

You may return the combinations in any order, and the numbers inside each combination in any order.

The inputs are chosen so that fewer than `150` unique combinations reach `target`.

<!-- examples -->

**Constraints:**

- `1 <= candidates.length <= 30`
- `2 <= candidates[i] <= 40`
- All elements of `candidates` are **distinct**.
- `1 <= target <= 40`
