class Solution:
    def floodFill(self, image: List[List[int]], sr: int, sc: int, color: int) -> List[List[int]]:
        original = image[sr][sc]
        if original == color:
            return image
        m, n = len(image), len(image[0])
        image[sr][sc] = color
        queue = deque([(sr, sc)])
        while queue:
            r, c = queue.popleft()
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < m and 0 <= nc < n and image[nr][nc] == original:
                    image[nr][nc] = color
                    queue.append((nr, nc))
        return image
