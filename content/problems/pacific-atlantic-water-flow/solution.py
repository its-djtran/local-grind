class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        m, n = len(heights), len(heights[0])

        def climb(starts):
            seen = [[False] * n for _ in range(m)]
            queue = deque()
            for r, c in starts:
                if not seen[r][c]:
                    seen[r][c] = True
                    queue.append((r, c))
            while queue:
                r, c = queue.popleft()
                h = heights[r][c]
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < m and 0 <= nc < n and not seen[nr][nc] and heights[nr][nc] >= h:
                        seen[nr][nc] = True
                        queue.append((nr, nc))
            return seen

        pacific = climb([(0, c) for c in range(n)] + [(r, 0) for r in range(m)])
        atlantic = climb([(m - 1, c) for c in range(n)] + [(r, n - 1) for r in range(m)])
        return [[r, c] for r in range(m) for c in range(n) if pacific[r][c] and atlantic[r][c]]
