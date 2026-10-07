/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

/**
 * @param {TreeNode} root
 * @param {TreeNode} subRoot
 * @return {boolean}
 */
var isSubtree = function(root, subRoot) {
    // Give every distinct subtree shape a unique integer id.
    const ids = new Map();
    let found = false;
    const ident = (node, target) => {
        if (!node) return 0;
        const key = `${ident(node.left, target)},${node.val},${ident(node.right, target)}`;
        let id = ids.get(key);
        if (id === undefined) {
            id = ids.size + 1;
            ids.set(key, id);
        }
        if (id === target) found = true;
        return id;
    };
    const target = ident(subRoot, -1);
    ident(root, target);
    return found;
};
