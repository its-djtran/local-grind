/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number[]}
 */
var findMinHeightTrees = function(n, edges) {
    if (n <= 2) return Array.from({ length: n }, (_, i) => i);
    const graph = Array.from({ length: n }, () => []);
    const degree = new Array(n).fill(0);
    for (const [a, b] of edges) {
        graph[a].push(b);
        graph[b].push(a);
        degree[a]++;
        degree[b]++;
    }
    let leaves = [];
    for (let v = 0; v < n; v++) if (degree[v] === 1) leaves.push(v);
    let remaining = n;
    while (remaining > 2) {
        remaining -= leaves.length;
        const next = [];
        for (const leaf of leaves) {
            for (const w of graph[leaf]) {
                if (--degree[w] === 1) next.push(w);
            }
        }
        leaves = next;
    }
    return leaves;
};
