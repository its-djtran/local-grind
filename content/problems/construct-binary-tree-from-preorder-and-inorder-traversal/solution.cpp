/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
    unordered_map<int, int> pos;
    int next = 0;

    TreeNode* build(const vector<int>& preorder, int lo, int hi) {
        if (lo > hi) return nullptr;
        TreeNode* node = new TreeNode(preorder[next++]);
        int k = pos[node->val];
        node->left = build(preorder, lo, k - 1);
        node->right = build(preorder, k + 1, hi);
        return node;
    }

public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        for (int i = 0; i < (int)inorder.size(); ++i) pos[inorder[i]] = i;
        return build(preorder, 0, (int)inorder.size() - 1);
    }
};
