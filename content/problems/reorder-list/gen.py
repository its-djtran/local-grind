import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(143)
N = 5 * 10**4


emit([1])
emit([1, 2])
emit([1, 2, 3])
emit([1000, 1])
emit([5, 5, 5, 5])
emit(list(range(1, 11)))
emit(list(range(1, 12)))
emit(list(range(20, 0, -1)))
# Every small length with distinct values, so misplaced nodes are visible.
for n in range(6, 30):
    emit(r.sample(range(1, 1001), n))
for _ in range(25):
    emit(int_array(r, r.randint(30, 3000), 1, 1000))
# Large cases: walking to the tail for every splice is quadratic.
for _ in range(3):
    emit(int_array(r, N, 1, 1000))
emit(int_array(r, N - 1, 1, 1000))
emit([(i % 1000) + 1 for i in range(N)])
