import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(73)
LO, HI = -(2**31), 2**31 - 1


def nonzero(lo, hi):
    while True:
        v = r.randint(lo, hi)
        if v != 0:
            return v


def grid(m, n, lo, hi, zero_prob):
    return [[0 if r.random() < zero_prob else nonzero(lo, hi) for _ in range(n)] for _ in range(m)]


def clustered(m, n, rows, cols, density, lo, hi):
    """Zeros only at intersections of a chosen set of rows and columns.

    Many zeros make a clear-per-zero solution slow, while the untouched rows and columns keep
    the expected output interesting.
    """
    g = [[nonzero(lo, hi) for _ in range(n)] for _ in range(m)]
    rs = r.sample(range(m), rows)
    cs = r.sample(range(n), cols)
    for i in rs:
        for j in cs:
            if r.random() < density:
                g[i][j] = 0
    return g


emit([[0]])
emit([[7]])
emit([[LO, HI]])
emit([[1], [0], [3]])
emit([[1, 0, 3]])
emit([[0, 0], [0, 0]])
emit([[1, 2], [3, 4]])
emit([[0, 1], [1, 1]])
emit([[1, 1], [1, 0]])
emit([[1, 0], [1, 1], [1, 1]])
emit([[1, 1, 1], [0, 1, 2]])
emit([[1, 2, 3, 4], [5, 0, 7, 8], [0, 10, 11, 12], [13, 14, 15, 0]])
emit([[HI, LO, 0], [LO, HI, 1], [-1, 1, HI]])
for _ in range(26):
    m, n = r.randint(1, 6), r.randint(1, 6)
    emit(grid(m, n, -5, 5, r.choice([0.05, 0.15, 0.3])))
for _ in range(15):
    m, n = r.randint(5, 60), r.randint(5, 60)
    emit(grid(m, n, LO, HI, r.choice([0.0, 0.01, 0.03])))
emit(grid(200, 200, LO, HI, 0.0))
emit(grid(1, 200, -1000, 1000, 0.02))
emit(grid(200, 1, -1000, 1000, 0.02))
emit(grid(200, 200, LO, HI, 0.00005))
for _ in range(5):
    emit(clustered(200, 200, 110, 110, 0.6, -1000, 1000))
# Dense zeros: a solution that clears a full row and column per zero does ~16M writes here.
for _ in range(12):
    emit(clustered(200, 200, r.randint(170, 199), r.randint(170, 199), r.uniform(0.8, 1.0), -9, 9))
