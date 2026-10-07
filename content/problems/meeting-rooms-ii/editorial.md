The rooms required equal the peak number of meetings in progress at once: at the busiest moment
every running meeting needs its own room, and a greedy assignment (below) never needs more.

## Approach 1: Count Overlaps at Every Start

**Intuition.** The peak is always reached at some meeting's start time. For each meeting, count
how many meetings are running at the instant it begins: those with `s <= start < e`.

```python
class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        best = 0
        for start, _ in intervals:
            running = sum(1 for s, e in intervals if s <= start < e)
            best = max(best, running)
        return best
```

- **Time:** `O(n^2)`.
- **Space:** `O(1)`.

## Approach 2: Min-Heap of End Times

**Intuition.** Assign meetings in order of start time. The only room worth reusing is the one
that becomes free the earliest; if even that room is still busy, every room is busy and a new
one is needed. A min-heap of end times gives the earliest-free room in `O(log n)`.

**Algorithm.**

1. Sort meetings by start.
2. For each meeting `[s, e]`: if the heap's smallest end time is `<= s`, pop it (that room is
   reused). Push `e`.
3. The heap's final size is the number of rooms opened.

```python
class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        intervals.sort()
        ends = []
        for s, e in intervals:
            if ends and ends[0] <= s:
                heapq.heapreplace(ends, e)
            else:
                heapq.heappush(ends, e)
        return len(ends)
```

- **Time:** `O(n log n)`.
- **Space:** `O(n)` for the heap.

## Approach 3: Sorted Starts and Ends (Optimal)

**Intuition.** Which meeting ends when does not matter, only *how many* have ended. Sort start
times and end times independently and sweep through the starts in order. Every end time that is
`<= ` the current start belongs to a meeting that has already released its room.

**Algorithm.**

1. Sort `starts` and `ends`.
2. Walk the starts with index `i` and keep a pointer `j` into `ends`. For each start, if
   `ends[j] <= starts[i]`, one meeting has finished, so advance `j` and reuse its room; otherwise
   open a new room.

```python
class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        starts = sorted(s for s, _ in intervals)
        ends = sorted(e for _, e in intervals)
        rooms = j = 0
        for s in starts:
            if ends[j] <= s:
                j += 1
            else:
                rooms += 1
        return rooms
```

- **Time:** `O(n log n)` for the two sorts; the sweep is linear.
- **Space:** `O(n)` for the sorted arrays.
