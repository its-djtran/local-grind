## Approach 1: Compare Against Every Earlier Node

**Intuition.** A list has a cycle exactly when the walk revisits a node. The most direct check
keeps the nodes seen so far in a plain list and scans it before each step.

**Algorithm.** Walk from `head`. For each node, scan the nodes already visited; if it is among
them (compared with `is`, since values may repeat), return `true`. If the walk reaches `None`,
return `false`.

```python
class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        visited = []
        node = head
        while node:
            for seen in visited:
                if seen is node:
                    return True
            visited.append(node)
            node = node.next
        return False
```

- **Time:** `O(n^2)` — the `k`-th step scans `k` earlier nodes.
- **Space:** `O(n)`.

## Approach 2: Hash Set of Visited Nodes

**Intuition.** Replace the linear scan with a hash set keyed by node identity, so "have I been
here?" costs `O(1)`.

**Algorithm.** Walk the list, adding each node to a set. If a node is already in the set, the
walk has looped; if the walk falls off the end, there is no cycle.

Store the nodes themselves (or `id(node)`), not their values: two different nodes may hold the
same value without forming a loop.

```python
class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        seen = set()
        node = head
        while node:
            if node in seen:
                return True
            seen.add(node)
            node = node.next
        return False
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the set.

## Approach 3: Fast and Slow Pointers (Optimal)

**Intuition.** Send two runners down the list: `slow` moves one node per step and `fast` moves
two. Without a cycle, `fast` reaches the end first. With a cycle, both runners eventually enter
the loop, and from then on the gap between them shrinks by exactly one node per step, so `fast`
must land on `slow` within one lap.

**Algorithm.**

1. Set `slow = fast = head`.
2. While `fast` and `fast.next` exist: advance `slow` by one and `fast` by two. If they are the
   same node, return `true`.
3. Return `false`.

```python
class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        slow = fast = head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            if slow is fast:
                return True
        return False
```

- **Time:** `O(n)` — `slow` takes at most about `n` steps before the pointers meet or `fast`
  exits.
- **Space:** `O(1)`.
