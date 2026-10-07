The order of the list hides a set of rules of the form "letter `x` comes before letter `y`".
Once those rules are extracted, the task is to order the letters so that every rule is respected,
which is exactly a **topological sort**.

**Extracting the rules.** Only adjacent words need to be compared: if every adjacent pair is in
order, the whole list is (the dictionary order is transitive). For a pair `a`, `b`:

- Find the first index where they differ. The letters there give the rule `a[i] -> b[i]`. Nothing
  after that index matters.
- If there is no differing index, one word is a prefix of the other. That is fine when
  `len(a) <= len(b)`, but if `b` is a *proper prefix* of `a` (like `"abc"` before `"ab"`), no
  alphabet can fix it and the answer is `""`.

Every letter that appears anywhere must be in the output, even if it takes part in no rule.

## Approach 1: Try Every Alphabet

**Intuition.** Generate every ordering of the distinct letters and test whether the list is sorted
under it.

```python
class Solution:
    def alienOrder(self, words: List[str]) -> str:
        letters = sorted(set("".join(words)))
        for perm in permutations(letters):
            rank = {c: i for i, c in enumerate(perm)}
            keys = [[rank[c] for c in w] for w in words]
            if all(keys[i] <= keys[i + 1] for i in range(len(keys) - 1)):
                return "".join(perm)
        return ""
```

- **Time:** `O(k! * C)` for `k` distinct letters and `C` total characters. Hopeless beyond about
  eight letters, and the alphabet can have twenty-six.
- **Space:** `O(C)`.

## Approach 2: DFS Topological Sort

**Intuition.** Build the graph of rules. A depth-first search finishes a letter only after
everything that must come after it is finished, so reversing the finishing order gives a valid
alphabet. Meeting a letter that is still on the DFS path reveals a cycle.

```python
class Solution:
    def alienOrder(self, words: List[str]) -> str:
        graph = {c: set() for w in words for c in w}
        for a, b in zip(words, words[1:]):
            for x, y in zip(a, b):
                if x != y:
                    graph[x].add(y)
                    break
            else:
                if len(a) > len(b):
                    return ""
        state, out = {}, []  # state: 1 = on the current path, 2 = finished

        def visit(c):
            state[c] = 1
            for d in graph[c]:
                if state.get(d) == 1 or (d not in state and not visit(d)):
                    return False
            state[c] = 2
            out.append(c)
            return True

        for c in graph:
            if c not in state and not visit(c):
                return ""
        return "".join(reversed(out))
```

- **Time:** `O(C + U)` where `C` is the total length of the words and `U <= 26` is the number of
  distinct letters (there are at most `min(n - 1, U^2)` edges).
- **Space:** `O(U^2)` for the graph, `O(U)` recursion.

## Approach 3: Kahn's Algorithm (Optimal)

**Intuition.** A letter with no incoming rule can safely go first. Output it, delete its outgoing
rules, and repeat. If letters remain but none is free, they form a cycle.

**Algorithm.**

1. Create a node for every letter that appears, then add one edge per adjacent pair as described
   above (returning `""` on a prefix violation). Use a set per node so repeated rules do not
   inflate in-degrees.
2. Queue every letter with in-degree `0`; repeatedly pop one, append it to the answer, and
   decrement the in-degrees of its successors.
3. If the answer contains every letter, return it; otherwise return `""`.

```python
class Solution:
    def alienOrder(self, words: List[str]) -> str:
        graph = {c: set() for w in words for c in w}
        indegree = {c: 0 for c in graph}
        for a, b in zip(words, words[1:]):
            for x, y in zip(a, b):
                if x != y:
                    if y not in graph[x]:
                        graph[x].add(y)
                        indegree[y] += 1
                    break
            else:
                if len(a) > len(b):
                    return ""
        queue = deque(c for c in graph if indegree[c] == 0)
        order = []
        while queue:
            c = queue.popleft()
            order.append(c)
            for d in graph[c]:
                indegree[d] -= 1
                if indegree[d] == 0:
                    queue.append(d)
        return "".join(order) if len(order) == len(graph) else ""
```

- **Time:** `O(C + U)` — every character is compared at most once while extracting rules.
- **Space:** `O(U^2)` in the worst case for the graph (`U <= 26`).
