## Approach 1: Per-Key History With a Linear Scan

**Intuition.** Store, for each key, the list of `(timestamp, value)` pairs in the order they
were set. To answer `get`, walk that list from the newest entry backwards and return the first
value whose timestamp is not after the query time.

```python
class TimeMap:
    def __init__(self):
        self.history = defaultdict(list)

    def set(self, key: str, value: str, timestamp: int) -> None:
        self.history[key].append((timestamp, value))

    def get(self, key: str, timestamp: int) -> str:
        for t, v in reversed(self.history.get(key, [])):
            if t <= timestamp:
                return v
        return ""
```

- **Time:** `O(1)` per `set`, but `O(k)` per `get`, where `k` is the number of entries for that
  key. Many reads of old times on a key with a long history add up to `O(n^2)`.
- **Space:** `O(n)` for all stored pairs.

## Approach 2: Per-Key History With Binary Search (Optimal)

**Intuition.** Because `set` timestamps strictly increase, each key's list is already sorted by
time. The answer to `get` is the last entry with timestamp `<= t`, which is exactly what a binary
search for the insertion point of `t` finds: `bisect_right(times, t)` is the number of entries at
or before `t`, so the entry just before that position is the answer, if there is one.

**Algorithm.**

1. Keep two parallel lists per key: `times[key]` and `values[key]`.
2. `set`: append the timestamp and the value.
3. `get`: let `i = bisect_right(times[key], timestamp)`. Return `values[key][i - 1]` when
   `i > 0`, otherwise `""`.

Keeping the timestamps in a separate list lets `bisect` compare plain integers.

```python
class TimeMap:
    def __init__(self):
        self.times = defaultdict(list)
        self.values = defaultdict(list)

    def set(self, key: str, value: str, timestamp: int) -> None:
        self.times[key].append(timestamp)
        self.values[key].append(value)

    def get(self, key: str, timestamp: int) -> str:
        times = self.times.get(key)
        if not times:
            return ""
        i = bisect_right(times, timestamp)
        return self.values[key][i - 1] if i else ""
```

- **Time:** `O(1)` amortized per `set` and `O(log k)` per `get`.
- **Space:** `O(n)` for all stored pairs.
