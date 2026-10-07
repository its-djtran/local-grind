/**
 * @param {number[][]} points
 * @param {number} k
 * @return {number[][]}
 */
var kClosest = function(points, k) {
    const n = points.length;
    const dist = new Int32Array(n);
    const idx = new Int32Array(n);
    for (let i = 0; i < n; i++) {
        const [x, y] = points[i];
        dist[i] = x * x + y * y;
        idx[i] = i;
    }
    const swap = (a, b) => { const t = idx[a]; idx[a] = idx[b]; idx[b] = t; };
    // Quickselect with a three-way partition: afterwards idx[0..k) are the k closest.
    let lo = 0, hi = n - 1;
    while (lo < hi) {
        const p = dist[idx[lo + Math.floor(Math.random() * (hi - lo + 1))]];
        let lt = lo, i = lo, gt = hi;
        while (i <= gt) {
            const d = dist[idx[i]];
            if (d < p) swap(lt++, i++);
            else if (d > p) swap(i, gt--);
            else i++;
        }
        // [lo, lt) < p, [lt, gt] == p, (gt, hi] > p
        if (k <= lt) hi = lt - 1;
        else if (k > gt + 1) lo = gt + 1;
        else break;
    }
    const result = new Array(k);
    for (let i = 0; i < k; i++) result[i] = points[idx[i]];
    return result;
};
