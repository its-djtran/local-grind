class LRUCache {
    int capacity;
    list<pair<int, int>> items; // most recently used at the front
    unordered_map<int, list<pair<int, int>>::iterator> pos;

public:
    LRUCache(int capacity) : capacity(capacity) {}

    int get(int key) {
        auto it = pos.find(key);
        if (it == pos.end()) return -1;
        items.splice(items.begin(), items, it->second);
        return it->second->second;
    }

    void put(int key, int value) {
        auto it = pos.find(key);
        if (it != pos.end()) {
            it->second->second = value;
            items.splice(items.begin(), items, it->second);
            return;
        }
        items.emplace_front(key, value);
        pos[key] = items.begin();
        if ((int)items.size() > capacity) {
            pos.erase(items.back().first);
            items.pop_back();
        }
    }
};
