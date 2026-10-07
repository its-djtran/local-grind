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
class Codec {
public:
    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        string out;
        queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            TreeNode* node = q.front();
            q.pop();
            if (!out.empty()) out += ',';
            if (!node) {
                out += '#';
            } else {
                out += to_string(node->val);
                q.push(node->left);
                q.push(node->right);
            }
        }
        return out;
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        vector<string> tokens;
        stringstream ss(data);
        for (string tok; getline(ss, tok, ',');) tokens.push_back(tok);
        if (tokens.empty() || tokens[0] == "#") return nullptr;
        TreeNode* root = new TreeNode(stoi(tokens[0]));
        queue<TreeNode*> q;
        q.push(root);
        for (size_t i = 1; !q.empty(); i += 2) {
            TreeNode* node = q.front();
            q.pop();
            if (tokens[i] != "#") {
                node->left = new TreeNode(stoi(tokens[i]));
                q.push(node->left);
            }
            if (tokens[i + 1] != "#") {
                node->right = new TreeNode(stoi(tokens[i + 1]));
                q.push(node->right);
            }
        }
        return root;
    }
};

// Your Codec object will be instantiated and called as such:
// Codec ser, deser;
// TreeNode* ans = deser.deserialize(ser.serialize(root));
