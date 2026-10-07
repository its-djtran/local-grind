import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(994)


def grid(m, n, weights):
    return [r.choices([0, 1, 2], weights, k=n) for _ in range(m)]


def all_reachable(m, n):
    """Random grid where every group of fresh oranges contains a rotten one (answer != -1)."""
    g = grid(m, n, [1, 5, 0])
    seen = set()
    for y in range(m):
        for x in range(n):
            if g[y][x] == 1 and (y, x) not in seen:
                comp, stack = [], [(y, x)]
                seen.add((y, x))
                while stack:
                    cy, cx = stack.pop()
                    comp.append((cy, cx))
                    for ny, nx in ((cy + 1, cx), (cy - 1, cx), (cy, cx + 1), (cy, cx - 1)):
                        if 0 <= ny < m and 0 <= nx < n and g[ny][nx] == 1 and (ny, nx) not in seen:
                            seen.add((ny, nx))
                            stack.append((ny, nx))
                for _ in range(r.randint(1, 2)):
                    cy, cx = r.choice(comp)
                    g[cy][cx] = 2
    return g


def simulate(g):
    """Minute-by-minute simulation, independent of the BFS reference."""
    g = [row[:] for row in g]
    m, n = len(g), len(g[0])
    minutes = 0
    while True:
        to_rot = [(y, x) for y in range(m) for x in range(n) if g[y][x] == 1 and any(
            0 <= y + dy < m and 0 <= x + dx < n and g[y + dy][x + dx] == 2
            for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)))]
        if not to_rot:
            break
        for y, x in to_rot:
            g[y][x] = 2
        minutes += 1
    return -1 if any(1 in row for row in g) else minutes


def snake(m, n):
    """Fresh oranges along one winding corridor with a single rotten orange at its start."""
    g = [[0] * n for _ in range(m)]
    for y in range(0, m, 2):
        g[y] = [1] * n
        if y + 1 < m:
            g[y + 1][n - 1 if (y // 2) % 2 == 0 else 0] = 1
    g[0][0] = 2
    return g


emit([[0]])
emit([[1]])
emit([[2]])
emit([[2, 1]])
emit([[1, 2, 1]])
emit([[1, 0, 2]])
emit([[2, 2], [2, 2]])
emit([[0, 0], [0, 0]])
emit([[1, 1], [1, 1]])
emit([[2, 1, 1], [1, 1, 1], [1, 1, 2]])
emit([[2, 0, 1]])
emit([[2], [1], [1], [1], [0], [1], [2]])
emit([[1, 1, 1], [1, 2, 1], [1, 1, 1]])
for _ in range(35):
    m, n = r.randint(1, 10), r.randint(1, 10)
    emit(grid(m, n, r.choice([[1, 3, 1], [1, 6, 1], [3, 3, 1], [0, 5, 1], [2, 8, 1]])))
for _ in range(12):
    emit(all_reachable(r.randint(1, 10), r.randint(1, 10)))
emit(snake(10, 10))
emit(snake(9, 10))
emit([[1] * 10 for _ in range(9)] + [[1] * 9 + [2]])
emit([[2] + [1] * 9] + [[1] * 10 for _ in range(9)])
emit([[1] * 10 for _ in range(10)])
emit([[2] * 10 for _ in range(10)])
g = [[1] * 10 for _ in range(10)]
g[0][0] = g[9][9] = 2
emit(g)
g = [[1] * 10 for _ in range(10)]
g[5][5] = 2
g[0][9] = 0
g[0][8] = g[1][9] = 0
emit(g)
