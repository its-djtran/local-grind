import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(53)
N, V = 10**5, 10**4

emit([-1])
emit([0])
emit([V])
emit([-V])
emit([-2, -1])
emit([-1, -2])
emit([-3, -5, -2, -8])  # all negative: answer is the largest single element
emit([-V] * 10)
emit([0, 0, 0])
emit([-1, 0, -2])
emit([1, -1, 1, -1, 1])
emit([3, -4, 5, -1, 2])
emit([2, -1, 2, -1, 2])
emit([8, -19, 5, -4, 20])
emit([V] * 10)
for n in range(2, 16):
    emit(int_array(r, n, -10, 10))
for _ in range(10):
    emit(int_array(r, r.randint(2, 30), -20, 5))  # mostly negative
for _ in range(20):
    emit(int_array(r, r.randint(30, 1500), -V, V))
# Large cases at the length limit.
emit(int_array(r, N, -V, V))
emit(int_array(r, N, -V, V // 2))
emit(int_array(r, N, -V, -1))
emit([V] * N)
emit([r.choice((V, -V + 1)) for _ in range(N)])
emit([-V] * (N // 2) + [V] + [-V] * (N // 2 - 1))
