## Approach 1: Hash Map Plus an Ordered Python List

**Intuition.** Keep the values in a dictionary and, separately, a list of keys ordered from least
to most recently used. Using a key means moving it to the end of the list; evicting means
removing the first key.

```python
class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.values = {}
        self.order = []  # least recent first

    def get(self, key: int) -> int:
        if key not in self.values:
            return -1
        self.order.remove(key)
        self.order.append(key)
        return self.values[key]

    def put(self, key: int, value: int) -> None:
        if key in self.values:
            self.order.remove(key)
        elif len(self.values) == self.capacity:
            del self.values[self.order.pop(0)]
        self.values[key] = value
        self.order.append(key)
```

- **Time:** `O(capacity)` per operation — `list.remove` and `list.pop(0)` shift up to `capacity`
  elements. That is too slow for `2 * 10^5` calls on a cache of thousands of entries.
- **Space:** `O(capacity)`.

## Approach 2: Hash Map Plus Doubly Linked List (Optimal)

**Intuition.** The recency order needs three operations to be `O(1)`: remove an arbitrary entry,
append at the most-recent end, and remove from the least-recent end. A doubly linked list does
all three, provided you can find a key's node instantly, which is what the hash map is for:
it maps each key to its node.

**Algorithm.**

- Keep two sentinel nodes, `head` (least recent side) and `tail` (most recent side), so every real
  node always has both neighbours.
- `get(key)`: if absent return `-1`; otherwise unlink the node, re-insert it before `tail`, and
  return its value.
- `put(key, value)`: if present, update the value and move the node before `tail`. Otherwise
  create a node, insert it before `tail`, and record it in the map; if the size now exceeds
  `capacity`, unlink `head.next` and delete its key from the map (each node stores its key for
  exactly this reason).

```python
class _Node:
    __slots__ = ("key", "val", "prev", "next")

    def __init__(self, key=0, val=0):
        self.key, self.val = key, val
        self.prev = self.next = None


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.nodes = {}
        self.head, self.tail = _Node(), _Node()
        self.head.next, self.tail.prev = self.tail, self.head

    def _unlink(self, node):
        node.prev.next, node.next.prev = node.next, node.prev

    def _append(self, node):
        last = self.tail.prev
        last.next = node
        node.prev, node.next = last, self.tail
        self.tail.prev = node

    def get(self, key: int) -> int:
        node = self.nodes.get(key)
        if node is None:
            return -1
        self._unlink(node)
        self._append(node)
        return node.val

    def put(self, key: int, value: int) -> None:
        node = self.nodes.get(key)
        if node is not None:
            node.val = value
            self._unlink(node)
            self._append(node)
            return
        node = _Node(key, value)
        self.nodes[key] = node
        self._append(node)
        if len(self.nodes) > self.capacity:
            oldest = self.head.next
            self._unlink(oldest)
            del self.nodes[oldest.key]
```

- **Time:** `O(1)` average per operation.
- **Space:** `O(capacity)`.

## Approach 3: OrderedDict

**Intuition.** Python's `collections.OrderedDict` is a hash map whose entries are threaded on a
doubly linked list, exactly the structure from Approach 2. `move_to_end(key)` marks a key as most
recent and `popitem(last=False)` removes the least recent entry, both in `O(1)`.

```python
class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = OrderedDict()

    def get(self, key: int) -> int:
        if key not in self.cache:
            return -1
        self.cache.move_to_end(key)
        return self.cache[key]

    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            self.cache.move_to_end(key)
        self.cache[key] = value
        if len(self.cache) > self.capacity:
            self.cache.popitem(last=False)
```

- **Time:** `O(1)` average per operation.
- **Space:** `O(capacity)`.

Interviewers often ask for Approach 2 explicitly, since it shows the underlying design.
