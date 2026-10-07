/**
 * // Definition for a _Node.
 * function _Node(val, neighbors) {
 *    this.val = val === undefined ? 0 : val;
 *    this.neighbors = neighbors === undefined ? [] : neighbors;
 * };
 */

/**
 * @param {_Node} node
 * @return {_Node}
 */
var cloneGraph = function(node) {
    if (!node) return null;
    const copies = new Map([[node, new _Node(node.val)]]);
    const queue = [node];
    for (let head = 0; head < queue.length; head++) {
        const cur = queue[head];
        const copy = copies.get(cur);
        for (const n of cur.neighbors) {
            if (!copies.has(n)) {
                copies.set(n, new _Node(n.val));
                queue.push(n);
            }
            copy.neighbors.push(copies.get(n));
        }
    }
    return copies.get(node);
};
