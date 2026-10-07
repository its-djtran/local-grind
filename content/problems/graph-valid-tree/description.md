You are given `n` nodes labeled `0` to `n - 1` and a list `edges`, where `edges[i] = [a, b]` is an undirected edge between nodes `a` and `b`.

Return `true` if these edges form a valid **tree** over all `n` nodes, meaning every node is reachable from every other node and there is no cycle. Otherwise return `false`.

<!-- examples -->

**Constraints:**

- `1 <= n <= 2000`
- `0 <= edges.length <= 5000`
- `edges[i].length == 2`
- `0 <= a, b < n`
- `a != b`
- No edge appears more than once (in either direction).
