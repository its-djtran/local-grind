/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};
*/
class Solution {
public:
    Node* cloneGraph(Node* node) {
        if (!node) return nullptr;
        unordered_map<Node*, Node*> copies{{node, new Node(node->val)}};
        queue<Node*> q;
        q.push(node);
        while (!q.empty()) {
            Node* cur = q.front();
            q.pop();
            for (Node* n : cur->neighbors) {
                auto it = copies.find(n);
                if (it == copies.end()) {
                    it = copies.emplace(n, new Node(n->val)).first;
                    q.push(n);
                }
                copies[cur]->neighbors.push_back(it->second);
            }
        }
        return copies[node];
    }
};
