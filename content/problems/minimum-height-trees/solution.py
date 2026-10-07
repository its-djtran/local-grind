class Solution:
    def findMinHeightTrees(self, n: int, edges: List[List[int]]) -> List[int]:
        if n <= 2:
            return list(range(n))
        graph = [[] for _ in range(n)]
        degree = [0] * n
        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)
            degree[a] += 1
            degree[b] += 1
        leaves = [v for v in range(n) if degree[v] == 1]
        remaining = n
        while remaining > 2:
            remaining -= len(leaves)
            nxt = []
            for leaf in leaves:
                for w in graph[leaf]:
                    degree[w] -= 1
                    if degree[w] == 1:
                        nxt.append(w)
            leaves = nxt
        return leaves
