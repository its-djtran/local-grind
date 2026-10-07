import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(141)
V = 10**5
N = 10**4

# Edge cases: empty list, single node with and without a self-loop, two nodes.
emit([], -1)
emit([1], -1)
emit([1], 0)
emit([1, 2], -1)
emit([1, 2], 0)
emit([1, 2], 1)
emit([-V, V], -1)
# Repeated values without a cycle: comparing values instead of nodes is wrong.
emit([5] * 10, -1)
emit([1, 2, 1, 2, 1, 2], -1)
emit([7] * 10, 9)
emit([7] * 10, 0)
emit(list(range(10)), 9)
emit(list(range(10)), 5)

# Small random cases, about half with a cycle.
for n in range(2, 32):
    vals = int_array(r, n, -5, 5) if n % 3 == 0 else int_array(r, n, -V, V)
    pos = r.randrange(n) if r.random() < 0.5 else -1
    emit(vals, pos)
for _ in range(20):
    n = r.randint(30, 2000)
    pos = r.randrange(n) if r.random() < 0.5 else -1
    emit(int_array(r, n, -V, V), pos)

# Large cases: an O(n^2) visited-list scan is too slow here.
emit(int_array(r, N, -V, V), -1)
emit(int_array(r, N, -V, V), N - 1)
emit(int_array(r, N, -V, V), 0)
emit(int_array(r, N, -V, V), N // 2)
emit([0] * N, -1)
emit(list(range(N)), -1)
