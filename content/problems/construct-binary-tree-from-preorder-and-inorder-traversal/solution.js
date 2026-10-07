/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

/**
 * @param {number[]} preorder
 * @param {number[]} inorder
 * @return {TreeNode}
 */
var buildTree = function(preorder, inorder) {
    const pos = new Map();
    inorder.forEach((v, i) => pos.set(v, i));
    let next = 0;
    const build = (lo, hi) => {
        if (lo > hi) return null;
        const node = new TreeNode(preorder[next++]);
        const k = pos.get(node.val);
        node.left = build(lo, k - 1);
        node.right = build(k + 1, hi);
        return node;
    };
    return build(0, inorder.length - 1);
};
