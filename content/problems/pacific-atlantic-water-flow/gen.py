import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(417)
H = 10**5


def grid(m, n, lo, hi):
    return [[r.randint(lo, hi) for _ in range(n)] for _ in range(m)]


def brute(heights):
    m, n = len(heights), len(heights[0])
    out = []
    for sr in range(m):
        for sc in range(n):
            pac = atl = False
            seen, stack = {(sr, sc)}, [(sr, sc)]
            while stack:
                y, x = stack.pop()
                pac |= y == 0 or x == 0
                atl |= y == m - 1 or x == n - 1
                for ny, nx in ((y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)):
                    if 0 <= ny < m and 0 <= nx < n and (ny, nx) not in seen and heights[ny][nx] <= heights[y][x]:
                        seen.add((ny, nx))
                        stack.append((ny, nx))
            if pac and atl:
                out.append([sr, sc])
    return out


def bowl(m, n):
    """Heights grow with distance from the centre, so water pools inward and inner cells reach
    huge regions without ever touching an ocean."""
    cy, cx = (m - 1) / 2, (n - 1) / 2
    return [[min(H, int(((y - cy) ** 2 + (x - cx) ** 2) * 4)) for x in range(n)] for y in range(m)]


def snake(m, n):
    """A single winding descending path: long flows and deep recursion for DFS solutions."""
    g = [[H] * n for _ in range(m)]
    h = H - 1
    for y in range(0, m, 2):
        cols = range(n) if (y // 2) % 2 == 0 else range(n - 1, -1, -1)
        for x in cols:
            g[y][x] = max(0, h)
            h -= 1
        if y + 1 < m:
            x = n - 1 if (y // 2) % 2 == 0 else 0
            g[y + 1][x] = max(0, h)
            h -= 1
    return g


small_cases = [
    [[1]],
    [[0]],
    [[1, 2, 3]],
    [[3], [2], [1]],
    [[5, 5], [5, 5]],
    [[1, 1], [1, 1], [1, 1]],
    [[10, 10, 10], [10, 1, 10], [10, 10, 10]],
    [[1, 2, 3], [8, 9, 4], [7, 6, 5]],
    [[3, 3, 3], [3, 0, 3], [3, 3, 3]],
    [[2, 1], [1, 2]],
    [[H, 0], [0, H]],
]
for g in small_cases:
    emit(g)
for _ in range(30):
    m, n = r.randint(1, 8), r.randint(1, 8)
    g = grid(m, n, 0, r.choice([2, 5, 20, H]))
    emit(g)
for _ in range(10):
    m, n = r.randint(9, 30), r.randint(9, 30)
    emit(grid(m, n, 0, r.choice([3, 10, H])))
emit(bowl(15, 17))
emit(snake(9, 11))

# Large cases: per-cell searches are far too slow on these.
emit(grid(200, 200, 0, H))
emit(grid(200, 200, 0, 3))
emit([[7] * 200 for _ in range(200)])
emit(bowl(200, 200))
emit([[H - v for v in row] for row in bowl(200, 200)])  # a central peak
emit(snake(200, 200))
emit([[y + x for x in range(200)] for y in range(200)])
emit([[(200 - y) * 300 + (200 - x) for x in range(200)] for y in range(200)])
emit(grid(1, 200, 0, H))
emit(grid(200, 1, 0, H))
