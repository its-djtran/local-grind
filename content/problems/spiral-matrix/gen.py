import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(54)


def grid(m, n):
    return [[r.randint(-100, 100) for _ in range(n)] for _ in range(m)]


emit([[5]])
emit([[-100]])
emit([[1, 2]])
emit([[1], [2]])
emit([[1, 2], [3, 4]])
emit([[1, 2, 3]])
emit([[1], [2], [3]])
emit([[1, 2], [3, 4], [5, 6]])
emit([[1, 2, 3], [4, 5, 6]])
emit([[i * 10 + j for j in range(10)] for i in range(10)])
emit([[0] * 10 for _ in range(10)])
emit([[100 if (i + j) % 2 else -100 for j in range(7)] for i in range(4)])
# Every shape once; small shapes catch boundary mistakes, odd shapes leave a middle row/column.
for m in range(1, 11):
    for n in range(1, 11):
        if m * n > 6:
            emit(grid(m, n))
