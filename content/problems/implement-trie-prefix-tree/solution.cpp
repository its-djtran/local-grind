class Trie {
    struct Node {
        Node* children[26] = {};
        bool end = false;
    };
    Node* root;

    Node* walk(const string& s) {
        Node* node = root;
        for (char ch : s) {
            node = node->children[ch - 'a'];
            if (!node) return nullptr;
        }
        return node;
    }

public:
    Trie() : root(new Node()) {}

    void insert(string word) {
        Node* node = root;
        for (char ch : word) {
            Node*& next = node->children[ch - 'a'];
            if (!next) next = new Node();
            node = next;
        }
        node->end = true;
    }

    bool search(string word) {
        Node* node = walk(word);
        return node && node->end;
    }

    bool startsWith(string prefix) {
        return walk(prefix) != nullptr;
    }
};
