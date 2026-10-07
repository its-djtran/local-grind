import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(15)
N, V = 3000, 10**5

emit([0, 0, 0, 0])
emit([1, 2, 3])
emit([-1, -2, -3])
emit([-1, 0, 1])
emit([1, -1, 0])
emit([0, 0, 1])
emit([-V, 0, V])
emit([V, V, V])
emit([-V, V // 2, V // 2])
emit([-2, 0, 1, 1, 2])
emit([-1, 0, 1, 0])
emit([3, 0, -2, -1, 1, 2])
emit([-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6])
emit([-1, -1, -1, 2, 2, 2])
emit([1, 1, -2])
emit([2, -1, -1, -1, -1])
emit([0] * 10 + [1, -1])
for n in range(3, 12):
    for _ in range(2):
        emit(int_array(r, n, -4, 4))
for _ in range(10):
    emit(int_array(r, r.randint(12, 40), -10, 10))
for _ in range(8):
    emit(int_array(r, r.randint(40, 300), -1000, 1000))
for _ in range(4):
    emit(int_array(r, r.randint(300, 1000), -V, V))
# Large cases at n = 3000 that force full O(n^2) work.
emit(int_array(r, N, -V, V))
emit([2 * r.randint(-V // 2, V // 2 - 1) + 1 for _ in range(N)])  # all odd: no triplet, no early exit
pool = int_array(r, 400, -V, V)
emit([r.choice(pool) for _ in range(N)])  # heavy duplicates
emit([0] * N)
emit([-1] * (N // 2) + [2] * (N // 2))
