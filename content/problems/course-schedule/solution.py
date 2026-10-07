class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        graph = [[] for _ in range(numCourses)]
        indegree = [0] * numCourses
        for a, b in prerequisites:
            graph[b].append(a)
            indegree[a] += 1
        queue = deque(v for v in range(numCourses) if indegree[v] == 0)
        taken = 0
        while queue:
            v = queue.popleft()
            taken += 1
            for w in graph[v]:
                indegree[w] -= 1
                if indegree[w] == 0:
                    queue.append(w)
        return taken == numCourses
