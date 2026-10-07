An undirected graph has `n` nodes labeled `0` to `n - 1`. You are given the integer `n` and a list `edges`, where `edges[i] = [a, b]` means there is an edge between nodes `a` and `b`.

Return the number of **connected components** in the graph: the number of groups of nodes such that nodes in the same group are linked by some path and nodes in different groups are not. A node with no edges is a component by itself.

<!-- examples -->

**Constraints:**

- `1 <= n <= 2000`
- `1 <= edges.length <= 5000`
- `edges[i].length == 2`
- `0 <= a <= b < n`
- `a != b`
- No edge is listed more than once.
