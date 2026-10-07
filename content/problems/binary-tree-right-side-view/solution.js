/**
 * @param {TreeNode} root
 * @return {number[]}
 */
var rightSideView = function(root) {
    const view = [];
    let level = root ? [root] : [];
    while (level.length) {
        view.push(level[level.length - 1].val);
        const next = [];
        for (const node of level) {
            if (node.left) next.push(node.left);
            if (node.right) next.push(node.right);
        }
        level = next;
    }
    return view;
};
