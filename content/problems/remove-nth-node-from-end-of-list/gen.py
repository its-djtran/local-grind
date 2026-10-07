import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(19)

emit([1, 2], 2)
emit([0], 1)
emit([100], 1)
emit([1, 2, 3], 3)
emit([1, 2, 3], 2)
emit([1, 2, 3], 1)
emit([7, 7, 7, 7], 2)
emit(list(range(30)), 30)
emit(list(range(30)), 1)
emit(list(range(30)), 15)
emit([100] * 30, 29)
# Every n for a few lengths, so removing the head, the tail and every middle node is covered.
for sz in (4, 5, 6):
    vals = int_array(r, sz, 0, 100)
    for n in range(1, sz + 1):
        emit(vals, n)
for _ in range(40):
    sz = r.randint(1, 30)
    emit(int_array(r, sz, 0, 100), r.randint(1, sz))
