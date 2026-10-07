import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(733)
MAXC = 2**16 - 1


def image(m, n, colors):
    return [[r.choice(colors) for _ in range(n)] for _ in range(m)]


def case(img, same=False):
    m, n = len(img), len(img[0])
    sr, sc = r.randrange(m), r.randrange(n)
    color = img[sr][sc] if same else r.choice([v for v in range(4) if v != img[sr][sc]] + [r.randint(0, MAXC)])
    emit(img, sr, sc, color)


def fill_brute(img, sr, sc, color):
    """Reference-independent check: compute the region by repeated relaxation."""
    m, n = len(img), len(img[0])
    orig = img[sr][sc]
    region = {(sr, sc)}
    changed = True
    while changed:
        changed = False
        for y in range(m):
            for x in range(n):
                if (y, x) not in region and img[y][x] == orig and any(
                        (y + dy, x + dx) in region for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1))):
                    region.add((y, x))
                    changed = True
    return [[color if (y, x) in region else img[y][x] for x in range(n)] for y in range(m)]


def snake(m, n):
    """Ones forming a single winding corridor separated by zero walls."""
    g = [[0] * n for _ in range(m)]
    for y in range(0, m, 2):
        g[y] = [1] * n
        if y + 1 < m:
            g[y + 1][n - 1 if (y // 2) % 2 == 0 else 0] = 1
    return g


emit([[0]], 0, 0, 0)
emit([[0]], 0, 0, 5)
emit([[MAXC]], 0, 0, 0)
emit([[1, 1, 1], [1, 1, 0], [1, 0, 1]], 2, 2, 2)
emit([[1, 0, 1], [0, 1, 0], [1, 0, 1]], 1, 1, 7)
emit([[2, 2], [2, 2]], 0, 1, 3)
emit([[1, 2, 3, 4, 5]], 0, 2, 9)
emit([[4], [4], [4], [5], [4]], 0, 0, 1)
emit([[0, 0, 0], [0, 1, 1]], 1, 1, 1)
emit([[0, 0, 0], [0, 1, 0]], 0, 0, 2)
for _ in range(30):
    m, n = r.randint(1, 8), r.randint(1, 8)
    case(image(m, n, r.choice([[0, 1], [0, 1, 2], [3, 7, 9, 11], list(range(6))])), same=r.random() < 0.15)
for _ in range(10):
    m, n = r.randint(9, 50), r.randint(9, 50)
    case(image(m, n, r.choice([[0, 1], [0, 1, 2], [5, MAXC]])), same=r.random() < 0.15)
emit([[1] * 50 for _ in range(50)], 25, 25, 2)
emit([[1] * 50 for _ in range(50)], 49, 0, 1)
emit([[MAXC] * 50 for _ in range(50)], 0, 0, 0)
emit(snake(50, 50), 0, 0, 3)
emit(snake(49, 50), 48, 49, 0)
emit([[(y + x) % 2 for x in range(50)] for y in range(50)], 10, 10, 2)
emit(image(50, 50, [0, 1]), 0, 0, 1)
