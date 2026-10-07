Picture a graph whose nodes are `beginWord` plus the words of `wordList`, with an edge between
two words that differ in exactly one position. A ladder is a path in this graph, and its length
in words is the number of edges plus one. Shortest paths in an unweighted graph are found by
**breadth-first search**; the approaches differ in how they find a word's neighbors. Let `N` be
the number of words and `L` their length.

## Approach 1: BFS with Pairwise Comparisons

**Intuition.** When a word is dequeued, compare it against every word that has not been visited
yet and enqueue those that differ in exactly one letter.

```python
class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        if endWord not in wordList:
            return 0
        unvisited = set(wordList)
        unvisited.discard(beginWord)
        queue = deque([(beginWord, 1)])
        while queue:
            word, steps = queue.popleft()
            if word == endWord:
                return steps
            nxt = [w for w in unvisited if sum(a != b for a, b in zip(word, w)) == 1]
            for w in nxt:
                unvisited.remove(w)
                queue.append((w, steps + 1))
        return 0
```

- **Time:** `O(N^2 * L)` — each dequeued word is compared letter by letter with up to `N` words.
  For `N = 5000` that is far too slow.
- **Space:** `O(N * L)`.

## Approach 2: BFS Generating Neighbors Letter by Letter

**Intuition.** A word has only `25 * L` possible one-letter variations. Generate them all and
keep the ones that are in a hash set of unvisited dictionary words.

```python
class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        words = set(wordList)
        if endWord not in words:
            return 0
        words.discard(beginWord)
        queue = deque([(beginWord, 1)])
        while queue:
            word, steps = queue.popleft()
            if word == endWord:
                return steps
            for i in range(len(word)):
                for ch in "abcdefghijklmnopqrstuvwxyz":
                    cand = word[:i] + ch + word[i + 1:]
                    if cand in words:
                        words.remove(cand)
                        queue.append((cand, steps + 1))
        return 0
```

- **Time:** `O(N * 26 * L^2)` — `26 * L` candidates per word, each built and hashed in `O(L)`.
- **Space:** `O(N * L)`.

## Approach 3: BFS over Wildcard Buckets (Optimal)

**Intuition.** Two words are neighbors exactly when they match the same *pattern* with one
position replaced by a wildcard: `hot` and `dot` both match `*ot`. Precompute, for each pattern,
the list of words matching it. A word's neighbors are then the union of its `L` buckets, and
the 26-letter loop disappears. Once a bucket has been expanded, every word in it has been
reached, so the bucket can be emptied to avoid scanning it again.

**Algorithm.**

1. If `endWord` is not in the list, return `0`.
2. Map every pattern `w[:i] + "*" + w[i+1:]` of every dictionary word to the words matching it.
3. BFS from `beginWord` level by level. For each dequeued word and each of its patterns, enqueue
   all unvisited words in that bucket, then clear the bucket.
4. Return the level at which `endWord` is reached, or `0` if the queue empties first.

```python
class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        if endWord not in wordList:
            return 0
        L = len(beginWord)
        buckets = defaultdict(list)
        for w in wordList:
            for i in range(L):
                buckets[w[:i] + "*" + w[i + 1:]].append(w)
        visited = {beginWord}
        queue = deque([beginWord])
        steps = 1
        while queue:
            for _ in range(len(queue)):
                word = queue.popleft()
                if word == endWord:
                    return steps
                for i in range(L):
                    key = word[:i] + "*" + word[i + 1:]
                    for nxt in buckets.pop(key, ()):
                        if nxt not in visited:
                            visited.add(nxt)
                            queue.append(nxt)
            steps += 1
        return 0
```

- **Time:** `O(N * L^2)` — each word creates `L` patterns of length `L`, and each bucket is
  expanded at most once.
- **Space:** `O(N * L^2)` for the buckets.

**Further speed-up.** Bidirectional BFS (expanding whichever frontier is smaller, from both
`beginWord` and `endWord`) explores far fewer words in practice, with the same worst case.
