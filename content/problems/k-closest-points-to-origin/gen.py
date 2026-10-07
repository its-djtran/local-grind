import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(973)
LIM = 10**4


def d2(p):
    return p[0] * p[0] + p[1] * p[1]


def unique(points, k):
    if k == len(points):
        return True
    ds = sorted(d2(p) for p in points)
    return ds[k - 1] < ds[k]


def rand_points(n, lim):
    return [[r.randint(-lim, lim), r.randint(-lim, lim)] for _ in range(n)]


def case(n, lim, k=None):
    while True:
        pts = rand_points(n, lim)
        kk = k if k is not None else r.randint(1, n)
        if unique(pts, kk):
            return pts, kk


def out(points, k):
    assert 1 <= k <= len(points) <= 10**4
    assert all(-LIM <= x <= LIM and -LIM <= y <= LIM for x, y in points)
    assert unique(points, k)
    emit(points, k)


out([[0, 0]], 1)
out([[LIM, LIM]], 1)
out([[1, 1], [2, 2]], 2)
out([[-LIM, -LIM], [LIM, LIM - 1]], 1)
out([[0, 1], [1, 0], [5, 5]], 2)
out([[1, 1], [1, 1], [5, 5]], 2)
out([[3, 4], [0, 5], [-5, 0], [1, 1]], 1)
out([[3, 4], [0, 5], [-5, 0], [1, 1]], 4)
out([[0, -LIM], [-LIM, 0], [LIM, 0], [0, LIM], [0, 0]], 1)
out([[2, 2], [2, 2], [3, 3], [2, -2], [-2, 2]], 4)
out([[-7, 0], [0, 6], [6, 6], [0, 0]], 3)
for n in range(1, 12):
    out(*case(n, 10))
for _ in range(30):
    n = r.randint(12, 500)
    out(*case(n, r.choice([10, 100, LIM])))
for _ in range(4):
    out(*case(r.randint(1000, LIM), LIM))
# Large cases: k close to n/2 punishes repeated selection.
for k in (5000, 4000, 6000, 3000, 7000, 5001):
    out(*case(LIM, LIM, k))
out(*case(LIM, LIM, 1))
out(*case(LIM, LIM, LIM))
