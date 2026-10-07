import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(876)

emit([1])
emit([1, 2])
emit([1, 2, 3])
emit([100, 1])
emit([7] * 10)
emit([7] * 11)
emit(list(range(1, 101)))
emit(list(range(1, 100)))
emit([100] * 100)
# Every length from 4 to 40 with distinct values, alternating odd and even.
for n in range(4, 41):
    emit(r.sample(range(1, 101), n))
for _ in range(20):
    emit(int_array(r, r.randint(1, 100), 1, 100))
