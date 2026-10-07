There are `n` jobs. Job `i` runs from time `startTime[i]` until time `endTime[i]` and pays `profit[i]` if you take it.

Choose a set of jobs in which no two jobs overlap in time, and return the **largest total profit** such a set can earn.

A job that ends at time `t` does not conflict with a job that starts at time `t`; you may take both.

<!-- examples -->

**Constraints:**

- `1 <= startTime.length == endTime.length == profit.length <= 5 * 10^4`
- `1 <= startTime[i] < endTime[i] <= 10^9`
- `1 <= profit[i] <= 10^4`
