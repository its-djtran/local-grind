import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(300)
V = 10**4

# Edge cases.
emit([5])
emit([-V])
emit([1, 2])
emit([2, 1])
emit([3, 3])
emit([-V, V])
emit([V, -V])
emit([4, 10, 4, 3, 8, 9])
emit([1, 3, 6, 7, 9, 4, 10, 5, 6])
emit([3, 5, 6, 2, 5, 4, 19, 5, 6, 7, 12])
emit([2, 2, 3, 3, 4, 4, 5])
emit(list(range(1, 21)))
emit(list(range(20, 0, -1)))

# Small random cases (narrow value ranges create many duplicates).
for _ in range(30):
    n = r.randint(1, 15)
    span = r.choice([3, 10, 100, V])
    emit(int_array(r, n, -span, span))

# Medium random cases.
for _ in range(15):
    emit(int_array(r, r.randint(16, 800), -V, V))

# Large cases at the limits.
for _ in range(3):
    emit(int_array(r, 2500, -V, V))
emit(list(range(-1250, 1250)))                     # answer 2500
emit(list(range(2500, 0, -1)))                     # answer 1
emit([7] * 2500)                                   # answer 1
emit(sorted(int_array(r, 2500, -V, V)))            # sorted with duplicates
emit([i // 2 + (i % 2) * 1000 for i in range(2500)])  # zig-zag
emit([max(-V, min(V, r.randint(-2000, 2000) + 8 * i - V)) for i in range(2500)])  # noisy upward trend
