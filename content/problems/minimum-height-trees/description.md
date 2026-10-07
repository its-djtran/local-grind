A **tree** is an undirected graph in which any two nodes are joined by exactly one path; equivalently, it is connected and has no cycles.

You are given a tree with `n` nodes labeled `0` to `n - 1` as a list of `n - 1` edges, where `edges[i] = [a, b]` connects nodes `a` and `b`. Any node can be chosen as the root. The **height** of the rooted tree is the number of edges on the longest downward path from the root to a leaf.

Among all possible roots, the ones that give the smallest height produce **minimum height trees**. Return the labels of all such roots, in any order.

<!-- examples -->

**Constraints:**

- `1 <= n <= 2 * 10^4`
- `edges.length == n - 1`
- `0 <= a, b < n`
- `a != b`
- All pairs `(a, b)` are distinct.
- The input is guaranteed to be a tree, and there are no repeated edges.
