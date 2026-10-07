import sys
from itertools import product

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(75)
N = 300

# Every array of length 1-3 (39 cases) covers all small orderings exhaustively.
for n in range(1, 4):
    for combo in product(range(3), repeat=n):
        emit(list(combo))
emit([2, 2, 0, 0])
emit([1, 2, 0, 1, 2, 0])
emit([2, 1, 1, 0])
emit([0, 0, 0, 0, 0])
emit([2, 2, 2, 2, 2])
emit([1, 1, 1, 1])
emit([2, 1, 0] * 5)
for _ in range(20):
    emit([r.randint(0, 2) for _ in range(r.randint(4, 40))])
for _ in range(6):
    k = r.randint(0, 2)
    emit([r.choice([v for v in range(3) if v != k]) for _ in range(r.randint(4, 40))])  # two colors
# Arrays at the length limit.
emit([r.randint(0, 2) for _ in range(N)])
emit([r.randint(0, 2) for _ in range(N)])
emit([2] * 100 + [1] * 100 + [0] * 100)
emit([0] * 100 + [1] * 100 + [2] * 100)
emit([2] * N)
emit([0] * (N - 1) + [2])
emit([2] + [0] * (N - 1))
