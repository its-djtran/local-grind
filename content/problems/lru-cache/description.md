Design a key-value cache with a fixed capacity that follows the **least recently used (LRU)** eviction policy.

Implement the `LRUCache` class:

- `LRUCache(int capacity)` creates an empty cache that can hold at most `capacity` entries (`capacity` is positive).
- `int get(int key)` returns the value stored for `key`, or `-1` if the key is not in the cache.
- `void put(int key, int value)` stores `value` under `key`, replacing the old value if the key is already present. If inserting a new key pushes the number of entries above `capacity`, first remove the entry that was used least recently.

Both a successful `get` and any `put` count as a "use" of that key. Each of `get` and `put` must run in `O(1)` average time.

The input is a list of operation names and a matching list of argument lists; the first operation constructs the cache. The output lists each operation's return value, with `null` for the constructor and `put`.

<!-- examples -->

**Constraints:**

- `1 <= capacity <= 3000`
- `0 <= key <= 10^4`
- `0 <= value <= 10^5`
- At most `2 * 10^5` calls are made to `get` and `put`.
