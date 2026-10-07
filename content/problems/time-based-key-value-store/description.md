Build a key-value store that remembers the full history of each key, so that you can ask what a key held at any moment in time.

Implement the `TimeMap` class:

- `TimeMap()` creates an empty store.
- `set(key, value, timestamp)` records that `key` took the value `value` at time `timestamp`.
- `get(key, timestamp)` returns the value of the most recent `set` for `key` whose time is **at most** `timestamp`. If `key` has never been set at or before that time, return the empty string `""`.

The timestamps passed to `set` are strictly increasing across all calls. Timestamps passed to `get` can be any value in range, in any order.

<!-- examples -->

**Constraints:**

- `1 <= key.length, value.length <= 100`
- `key` and `value` contain only lowercase English letters and digits.
- `1 <= timestamp <= 10^7`
- The timestamps of all `set` calls are strictly increasing.
- At most `2 * 10^5` calls are made to `set` and `get` in total.
