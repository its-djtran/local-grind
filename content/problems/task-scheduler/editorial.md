Only the count of each label matters. Let `m` be the number of tasks.

## Approach 1: Slot-by-Slot Greedy Simulation

**Intuition.** At every slot, run the available label (not cooling down) with the most copies
left. Delaying the most frequent label is what causes idle time later, so it should be served
first whenever possible. If nothing is available, idle.

**Algorithm.** Keep `count[c]` and `ready[c]` (the first slot where label `c` may run again).
For `t = 0, 1, 2, ...` until all counts are zero, pick the label with `ready[c] <= t` and the
largest `count[c]`; run it and set `ready[c] = t + n + 1`.

```python
class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        count = Counter(tasks)
        ready = {c: 0 for c in count}
        left, t = len(tasks), 0
        while left:
            best = None
            for c in count:
                if count[c] and ready[c] <= t and (best is None or count[c] > count[best]):
                    best = c
            if best is not None:
                count[best] -= 1
                ready[best] = t + n + 1
                left -= 1
            t += 1
        return t
```

- **Time:** `O(T * 26)` where `T` is the answer, up to about `m * (n + 1)`, i.e. roughly
  `10^6` slots for `10^4` copies of one task with `n = 100`.
- **Space:** `O(1)` (26 labels).

## Approach 2: Max-Heap with a Cooldown Queue

**Intuition.** The same greedy, but instead of scanning every slot we keep available labels in a
max-heap by remaining count and cooling labels in a FIFO queue tagged with the slot they become
ready. When the heap is empty we jump the clock straight to the next ready time instead of
stepping through idle slots one by one.

```python
class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        heap = [-c for c in Counter(tasks).values()]
        heapq.heapify(heap)
        cooling = deque()  # (ready_time, negative_count)
        t = 0
        while heap or cooling:
            if not heap:
                t = max(t, cooling[0][0])
            while cooling and cooling[0][0] <= t:
                heapq.heappush(heap, cooling.popleft()[1])
            c = heapq.heappop(heap) + 1
            if c:
                cooling.append((t + n + 1, c))
            t += 1
        return t
```

- **Time:** `O(m log 26) = O(m)` — one heap operation per task.
- **Space:** `O(1)`.

## Approach 3: Counting Formula (Optimal)

**Intuition.** Let `f` be the highest count and `k` the number of labels that have it. Picture
the `f` copies of a most frequent label as the start of `f` frames, each `n + 1` slots long;
the last frame only needs to hold the `k` top labels. That skeleton has
`(f - 1) * (n + 1) + k` slots, and every other task fits in the gaps of the first `f - 1`
frames (each label appears at most once per frame, so its copies stay `n + 1` apart). No
schedule can be shorter: the copies of a top label alone span `(f - 1) * (n + 1) + 1` slots,
and the other `k - 1` top labels each need one more slot after it in the final frame.

If the gaps overflow — more tasks than skeleton slots — the frames simply grow wider and no slot
is idle, so the answer becomes `m`. Combining both cases:

`answer = max(m, (f - 1) * (n + 1) + k)`

```python
class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        counts = Counter(tasks).values()
        f = max(counts)
        k = sum(1 for c in counts if c == f)
        return max(len(tasks), (f - 1) * (n + 1) + k)
```

- **Time:** `O(m)` to count.
- **Space:** `O(1)` (at most 26 labels).
