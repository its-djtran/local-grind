The region to repaint is the set of pixels with the starting color that are connected to the start
through horizontal and vertical steps. Any graph traversal of the grid finds it.

One trap applies to every approach: if `color` equals the original color, repainting a pixel does
not change it, so a search that relies on "this pixel is already the new color" to avoid revisits
would loop forever. Since the image would not change anyway, return it immediately in that case.

## Approach 1: Recursive Depth-First Search

**Intuition.** Repaint the current pixel, then recurse into each neighbor that still has the
original color. A repainted pixel no longer matches `original`, so it is never visited twice.

```python
class Solution:
    def floodFill(self, image: List[List[int]], sr: int, sc: int, color: int) -> List[List[int]]:
        original = image[sr][sc]
        if original == color:
            return image
        m, n = len(image), len(image[0])

        def fill(r, c):
            if 0 <= r < m and 0 <= c < n and image[r][c] == original:
                image[r][c] = color
                fill(r + 1, c)
                fill(r - 1, c)
                fill(r, c + 1)
                fill(r, c - 1)

        fill(sr, sc)
        return image
```

- **Time:** `O(m * n)` — each pixel is repainted at most once and checked from at most four
  neighbors.
- **Space:** `O(m * n)` for the recursion in the worst case (a region snaking through the whole
  image).

## Approach 2: Breadth-First Search (Optimal)

**Intuition.** The same fill with a queue: repaint a pixel when it is enqueued, which both marks it
visited and guarantees it is enqueued only once. No recursion depth to worry about.

```python
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
```

- **Time:** `O(m * n)`.
- **Space:** `O(m * n)` for the queue in the worst case.
