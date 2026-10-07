// Doubly linked list stored in a Vec and addressed by index. Slots 0 and 1 are the
// head and tail sentinels; the most recently used entry sits right after the head.
struct Node {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
}

const HEAD: usize = 0;
const TAIL: usize = 1;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let mut nodes = Vec::with_capacity(capacity + 2);
        nodes.push(Node { key: 0, val: 0, prev: HEAD, next: TAIL });
        nodes.push(Node { key: 0, val: 0, prev: HEAD, next: TAIL });
        LRUCache { capacity, map: HashMap::with_capacity(capacity), nodes }
    }

    fn unlink(&mut self, i: usize) {
        let (prev, next) = (self.nodes[i].prev, self.nodes[i].next);
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }

    fn push_front(&mut self, i: usize) {
        let first = self.nodes[HEAD].next;
        self.nodes[i].prev = HEAD;
        self.nodes[i].next = first;
        self.nodes[first].prev = i;
        self.nodes[HEAD].next = i;
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(&i) => {
                self.unlink(i);
                self.push_front(i);
                self.nodes[i].val
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].val = value;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        let i = if self.map.len() == self.capacity {
            // Reuse the least recently used slot.
            let lru = self.nodes[TAIL].prev;
            self.unlink(lru);
            self.map.remove(&self.nodes[lru].key);
            self.nodes[lru].key = key;
            self.nodes[lru].val = value;
            lru
        } else {
            self.nodes.push(Node { key, val: value, prev: HEAD, next: TAIL });
            self.nodes.len() - 1
        };
        self.map.insert(key, i);
        self.push_front(i);
    }
}
