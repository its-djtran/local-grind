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
