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
