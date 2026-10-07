A histogram is given as an array `heights`, where bar `i` has width `1` and height `heights[i]`, and neighbouring bars touch.

Return the area of the largest axis-aligned rectangle that fits entirely inside the histogram. The rectangle must sit on the baseline and span one or more consecutive bars, so its height can be no greater than the shortest bar it covers.

<!-- examples -->

**Constraints:**

- `1 <= heights.length <= 10^5`
- `0 <= heights[i] <= 10^4`
