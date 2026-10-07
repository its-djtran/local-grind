class Solution {
public:
    bool isValidBST(TreeNode* root) {
        vector<TreeNode*> stack;
        TreeNode* node = root;
        long long prev = LLONG_MIN;
        while (!stack.empty() || node) {
            while (node) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            if (node->val <= prev) return false;
            prev = node->val;
            node = node->right;
        }
        return true;
    }
};
