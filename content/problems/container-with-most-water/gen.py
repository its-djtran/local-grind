import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(11)
N, V = 10**5, 10**4

emit([0, 0])
emit([0, V])
emit([V, V])
emit([1, 2])
emit([2, 1])
emit([4, 3, 2, 1, 4])
emit([1, 2, 1])
emit([1, 2, 4, 3])
emit([2, 3, 4, 5, 18, 17, 6])
emit([1, 3, 2, 5, 25, 24, 5])
emit([0, 0, 0, 0, 0])
emit([1, 100, 100, 1])
emit([5, 5, 5, 5, 5, 5])
emit(list(range(1, 11)))
emit(list(range(10, 0, -1)))
for n in range(2, 16):
    emit(int_array(r, n, 0, 20))
for _ in range(25):
    emit(int_array(r, r.randint(16, 1500), 0, V))
# Large cases: random, monotone, tall lines hidden in the middle, all equal.
emit(int_array(r, N, 0, V))
emit(int_array(r, N, 0, V))
emit([i // 10 for i in range(N)])
emit([V - i // 10 for i in range(N)])
mid = [r.randint(0, 50) for _ in range(N)]
mid[N // 2 - 3], mid[N // 2 + 3] = V, V
emit(mid)
emit([V] * N)
