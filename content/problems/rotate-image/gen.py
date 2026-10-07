import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(48)


def grid(n, lo=-1000, hi=1000):
    return [[r.randint(lo, hi) for _ in range(n)] for _ in range(n)]


emit([[1]])
emit([[-1000]])
emit([[1, 2], [3, 4]])
emit([[1000, -1000], [-1000, 1000]])
emit([[0, 0], [0, 0]])
emit([[i * 20 + j for j in range(20)] for i in range(20)])
emit([[7] * 5 for _ in range(5)])
emit([[1 if i == j else 0 for j in range(6)] for i in range(6)])
emit([[1 if j == 0 else 0 for j in range(5)] for i in range(5)])
for n in range(2, 21):
    emit(grid(n, -9, 9))
for _ in range(25):
    emit(grid(r.randint(1, 20)))
for _ in range(5):
    emit(grid(20))
