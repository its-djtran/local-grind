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
 * @return {boolean}
 */
var isBalanced = function(root) {
    // Height of the subtree, or -1 if it is unbalanced.
    const check = (node) => {
        if (!node) return 0;
        const left = check(node.left);
        if (left < 0) return -1;
        const right = check(node.right);
        if (right < 0 || Math.abs(left - right) > 1) return -1;
        return 1 + Math.max(left, right);
    };
    return check(root) >= 0;
};
