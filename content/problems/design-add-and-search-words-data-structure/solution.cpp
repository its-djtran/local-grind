class WordDictionary {
    struct Node {
        Node* children[26] = {};
        bool end = false;
    };
    Node* root;

public:
    WordDictionary() : root(new Node()) {}

    void addWord(string word) {
        Node* node = root;
        for (char ch : word) {
            Node*& next = node->children[ch - 'a'];
            if (!next) next = new Node();
            node = next;
        }
        node->end = true;
    }

    bool search(string word) {
        vector<Node*> level{root}, next;
        for (char ch : word) {
            next.clear();
            for (Node* node : level) {
                if (ch == '.') {
                    for (Node* child : node->children)
                        if (child) next.push_back(child);
                } else if (Node* child = node->children[ch - 'a']) {
                    next.push_back(child);
                }
            }
            if (next.empty()) return false;
            swap(level, next);
        }
        return any_of(level.begin(), level.end(), [](Node* node) { return node->end; });
    }
};
