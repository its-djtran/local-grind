You are given a list of points on the plane, `points[i] = [x_i, y_i]`, and an integer `k`. Return the `k` points that lie **closest to the origin** `(0, 0)`.

Distance is the usual straight-line (Euclidean) distance, `sqrt(x^2 + y^2)`.

The points may be returned in any order. The tests are built so that the set of `k` closest points is uniquely determined.

<!-- examples -->

**Constraints:**

- `1 <= k <= points.length <= 10^4`
- `-10^4 <= x_i, y_i <= 10^4`
