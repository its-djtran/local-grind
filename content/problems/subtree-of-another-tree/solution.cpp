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
    // Give every distinct subtree shape a unique integer id.
    struct KeyHash {
        size_t operator()(const tuple<int, int, int>& k) const {
            auto [l, v, r] = k;
            uint64_t h = (uint64_t(uint32_t(l)) << 32) ^ uint32_t(r);
            h ^= uint64_t(uint32_t(v)) * 0x9E3779B97F4A7C15ULL;
            h ^= h >> 29;
            return h * 0xBF58476D1CE4E5B9ULL;
        }
    };
    unordered_map<tuple<int, int, int>, int, KeyHash> ids;
    bool found = false;

    int ident(TreeNode* node, int target) {
        if (!node) return 0;
        tuple<int, int, int> key{ident(node->left, target), node->val, ident(node->right, target)};
        auto [it, inserted] = ids.try_emplace(key, ids.size() + 1);
        if (it->second == target) found = true;
        return it->second;
    }

public:
    bool isSubtree(TreeNode* root, TreeNode* subRoot) {
        int target = ident(subRoot, -1);
        ident(root, target);
        return found;
    }
};
