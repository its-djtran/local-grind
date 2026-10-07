/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

/**
 * Encodes a tree to a single string.
 *
 * @param {TreeNode} root
 * @return {string}
 */
var serialize = function(root) {
    const out = [];
    const queue = [root];
    for (let head = 0; head < queue.length; head++) {
        const node = queue[head];
        if (!node) {
            out.push('#');
        } else {
            out.push(node.val);
            queue.push(node.left, node.right);
        }
    }
    return out.join(',');
};

/**
 * Decodes your encoded data to tree.
 *
 * @param {string} data
 * @return {TreeNode}
 */
var deserialize = function(data) {
    const tokens = data.split(',');
    if (!data || tokens[0] === '#') return null;
    const root = new TreeNode(+tokens[0]);
    const queue = [root];
    let i = 1;
    for (let head = 0; head < queue.length; head++, i += 2) {
        const node = queue[head];
        if (tokens[i] !== '#') {
            node.left = new TreeNode(+tokens[i]);
            queue.push(node.left);
        }
        if (tokens[i + 1] !== '#') {
            node.right = new TreeNode(+tokens[i + 1]);
            queue.push(node.right);
        }
    }
    return root;
};

/**
 * Your functions will be called as such:
 * deserialize(serialize(input));
 */
