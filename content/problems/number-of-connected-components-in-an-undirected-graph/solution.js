/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number}
 */
var countComponents = function(n, edges) {
    const parent = Array.from({ length: n }, (_, i) => i);
    const size = new Array(n).fill(1);
    const find = (x) => {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    };
    let count = n;
    for (const [a, b] of edges) {
        let ra = find(a), rb = find(b);
        if (ra === rb) continue;
        if (size[ra] < size[rb]) [ra, rb] = [rb, ra];
        parent[rb] = ra;
        size[ra] += size[rb];
        count--;
    }
    return count;
};
