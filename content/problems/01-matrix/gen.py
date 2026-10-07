import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(542)


def matrix(m, n, p_zero):
    """Random 0/1 matrix with at least one zero (retries until the guarantee holds)."""
    while True:
        g = [[0 if r.random() < p_zero else 1 for _ in range(n)] for _ in range(m)]
        if any(0 in row for row in g):
            return g


def ones_with_zeros(m, n, zeros):
    g = [[1] * n for _ in range(m)]
    for y, x in zeros:
        g[y][x] = 0
    return g


def dist_brute(g):
    m, n = len(g), len(g[0])
    zeros = [(y, x) for y in range(m) for x in range(n) if g[y][x] == 0]
    return [[min(abs(y - a) + abs(x - b) for a, b in zeros) for x in range(n)] for y in range(m)]


emit([[0]])
emit([[0, 1]])
emit([[1], [0]])
emit([[1, 1, 1, 0]])
emit([[0], [1], [1], [1], [1]])
emit([[0, 0], [0, 0]])
emit([[1, 1, 1], [1, 0, 1], [1, 1, 1]])
emit([[0, 1, 1], [1, 1, 1], [1, 1, 1]])
emit([[1, 1, 1], [1, 1, 1], [1, 1, 0]])
emit([[0, 1, 0, 1, 1], [1, 1, 0, 0, 1], [0, 0, 0, 1, 0], [1, 0, 1, 1, 1], [1, 0, 0, 0, 1]])
for _ in range(30):
    m, n = r.randint(1, 10), r.randint(1, 10)
    emit(matrix(m, n, r.choice([0.05, 0.2, 0.5, 0.8])))
for _ in range(10):
    m = r.randint(10, 100)
    n = r.randint(10, 10**4 // m)
    emit(matrix(m, n, r.choice([0.01, 0.1, 0.3])))

# Large: m * n = 10^4 with very few zeros, so a search from each 1 travels far.
emit(ones_with_zeros(100, 100, [(0, 0)]))
emit(ones_with_zeros(100, 100, [(99, 99)]))
emit(ones_with_zeros(100, 100, [(50, 50)]))
emit(ones_with_zeros(1, 10**4, [(0, 0)]))
emit(ones_with_zeros(10**4, 1, [(9999, 0)]))
emit(ones_with_zeros(50, 200, [(0, 199), (49, 0)]))
emit(ones_with_zeros(100, 100, [(r.randrange(100), r.randrange(100)) for _ in range(3)]))
emit(matrix(100, 100, 0.5))
emit([[0] * 100 for _ in range(100)])
emit(matrix(25, 400, 0.002))
