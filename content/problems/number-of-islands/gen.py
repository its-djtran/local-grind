import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(200)


def grid(m, n, p):
    return [["1" if r.random() < p else "0" for _ in range(n)] for _ in range(m)]


def text(rows):
    return [list(row) for row in rows]


def islands_brute(g):
    """Union-find over all land cells; independent of the reference's flood fill."""
    m, n = len(g), len(g[0])
    parent = {}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for y in range(m):
        for x in range(n):
            if g[y][x] == "1":
                parent[(y, x)] = (y, x)
    for y in range(m):
        for x in range(n):
            if g[y][x] == "1":
                for ny, nx in ((y + 1, x), (y, x + 1)):
                    if ny < m and nx < n and g[ny][nx] == "1":
                        parent[find((y, x))] = find((ny, nx))
    return len({find(k) for k in parent})


def spiral(m, n):
    """One spiral-shaped island: a single very long path of land."""
    g = [["0"] * n for _ in range(m)]
    top, left, bottom, right = 0, 0, m - 1, n - 1
    while top <= bottom and left <= right:
        for x in range(left, right + 1):
            g[top][x] = "1"
        for y in range(top, bottom + 1):
            g[y][right] = "1"
        if top + 2 <= bottom:
            for x in range(left, right + 1):
                g[bottom][x] = "1"
        if left + 2 <= right and top + 2 <= bottom:
            for y in range(top + 2, bottom + 1):
                g[y][left] = "1"
            if top + 2 < bottom:
                g[top + 2][left + 1] = "1" if left + 1 <= right else "0"
        top, left, bottom, right = top + 2, left + 2, bottom - 2, right - 2
    return g


def checker(m, n):
    return [["1" if (y + x) % 2 == 0 else "0" for x in range(n)] for y in range(m)]


def snake_rows(m, n):
    """Full land rows on even indices joined alternately at the right and left ends."""
    g = [["0"] * n for _ in range(m)]
    for y in range(0, m, 2):
        g[y] = ["1"] * n
        if y + 1 < m:
            g[y + 1][n - 1 if (y // 2) % 2 == 0 else 0] = "1"
    return g


emit([["1"]])
emit([["0"]])
emit(text(["10101"]))
emit(text(["1", "0", "1", "1"]))
emit(text(["11", "11"]))
emit(text(["101", "010", "101"]))
emit(text(["111", "101", "111"]))
emit(text(["010", "111", "010"]))
emit(text(["000", "000"]))
emit(text(["11000", "11000", "00100", "00011"]))
for _ in range(30):
    m, n = r.randint(1, 10), r.randint(1, 10)
    emit(grid(m, n, r.choice([0.2, 0.4, 0.5, 0.6, 0.8])))
for _ in range(10):
    m, n = r.randint(10, 80), r.randint(10, 80)
    emit(grid(m, n, r.choice([0.3, 0.45, 0.6])))
emit(spiral(13, 17))
emit(snake_rows(9, 6))

emit([["1"] * 300 for _ in range(300)])
emit([["0"] * 300 for _ in range(300)])
emit(checker(300, 300))
emit(spiral(300, 300))
emit(snake_rows(300, 300))
emit(grid(300, 300, 0.5))
emit(grid(300, 300, 0.6))
emit(grid(1, 300, 0.5))
emit(grid(300, 1, 0.5))
