import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(21)


def sorted_list(n, lo=-100, hi=100):
    return sorted(r.randint(lo, hi) for _ in range(n))


emit([], [1])
emit([1], [])
emit([5], [1])
emit([1], [5])
emit([2], [2])
emit([-100], [100])
emit([100] * 50, [-100] * 50)
emit([-100] * 50, [100] * 50)
emit(list(range(-100, 100, 4)), list(range(-98, 100, 4)))
emit([1, 2, 3], [4, 5, 6])
emit([4, 5, 6], [1, 2, 3])
emit([1, 1, 1], [1, 1])
emit([0] * 50, [0] * 50)
emit(list(range(50)), [])
emit([], list(range(-50, 0)))
# Small random cases, including empty sides and narrow value ranges (many ties).
for _ in range(30):
    emit(sorted_list(r.randint(0, 8), -5, 5), sorted_list(r.randint(0, 8), -5, 5))
for _ in range(20):
    emit(sorted_list(r.randint(0, 50)), sorted_list(r.randint(0, 50)))
for _ in range(5):
    emit(sorted_list(50), sorted_list(50))
