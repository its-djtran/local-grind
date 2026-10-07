import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(153)
N, V = 5000, 5000


def rotated(n, k=None, lo=-V, hi=V):
    vals = sorted(r.sample(range(lo, hi + 1), n))
    if k is None:
        k = r.randrange(n)
    k %= n
    return vals[n - k:] + vals[: n - k]  # k right-rotations


emit([1])
emit([-V])
emit([V])
emit([1, 2])
emit([2, 1])
emit([-V, V])
emit([V, -V])
emit([1, 2, 3])
emit([3, 1, 2])
emit([2, 3, 1])
emit([5, 1, 2, 3, 4])
emit([2, 3, 4, 5, 1])
emit([-3, -2, -1])
emit([0, 1, 2, -2, -1])
for n in range(1, 13):
    for k in range(n):
        if r.random() < 0.35 or k in (0, 1, n - 1):
            emit(rotated(n, k, -20, 20))
for _ in range(10):
    emit(rotated(r.randint(13, 500)))
for k in (0, 1, N // 2, N - 1, r.randrange(N), r.randrange(N)):
    emit(rotated(N, k))
emit(list(range(-V // 2, V // 2)))
emit(list(range(1, N)) + [-V])  # minimum is the very last element
