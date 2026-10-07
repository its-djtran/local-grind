import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(198)

# Edge cases.
emit([0])
emit([400])
emit([1, 2])
emit([2, 1])
emit([0, 0, 0])
emit([2, 1, 1, 2])          # alternating sums give 3, the answer is 4
emit([5, 1, 1, 5])
emit([1, 3, 1, 3, 100])
emit([400] * 100)
emit([0] * 100)
emit([400, 0] * 50)
emit([1, 400, 1, 1, 400, 1] * 16 + [1, 400, 1, 1])

# Small random cases.
for n in range(3, 23):
    emit(int_array(r, n, 0, r.choice([5, 50, 400])))
for _ in range(15):
    emit(int_array(r, r.randint(3, 12), 0, 10))

# Medium random cases.
for _ in range(8):
    emit(int_array(r, r.randint(23, 99), 0, 400))

# Large cases at the limit: plain recursion explores ~1.6^100 paths.
for _ in range(5):
    emit(int_array(r, 100, 0, 400))
emit([r.choice([0, 400]) for _ in range(100)])
emit(list(range(100)))
emit(list(range(400, 300, -1)))
