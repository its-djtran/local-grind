/**
 * @param {string[]} words
 * @return {string}
 */
var alienOrder = function(words) {
    const graph = new Map();
    const indegree = new Map();
    for (const w of words) {
        for (const c of w) {
            if (!graph.has(c)) {
                graph.set(c, new Set());
                indegree.set(c, 0);
            }
        }
    }
    for (let i = 0; i + 1 < words.length; i++) {
        const a = words[i], b = words[i + 1];
        const len = Math.min(a.length, b.length);
        let j = 0;
        while (j < len && a[j] === b[j]) j++;
        if (j < len) {
            const x = a[j], y = b[j];
            if (!graph.get(x).has(y)) {
                graph.get(x).add(y);
                indegree.set(y, indegree.get(y) + 1);
            }
        } else if (a.length > b.length) {
            return "";
        }
    }
    const queue = [];
    for (const [c, d] of indegree) if (d === 0) queue.push(c);
    for (let head = 0; head < queue.length; head++) {
        for (const d of graph.get(queue[head])) {
            indegree.set(d, indegree.get(d) - 1);
            if (indegree.get(d) === 0) queue.push(d);
        }
    }
    return queue.length === graph.size ? queue.join("") : "";
};
