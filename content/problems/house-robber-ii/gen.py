import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(213)

# Edge cases.
emit([7])
emit([0])
emit([1000])
emit([1, 2])
emit([5, 5])
emit([0, 0, 0])
emit([200, 3, 140, 20, 10])
emit([1, 3, 1, 3, 100])     # first and last both look attractive
emit([4, 1, 2, 7, 5, 3, 1])
emit([2, 1, 1, 2])
emit([1000] * 100)
emit([1000] * 99)
emit([1000, 0] * 50)
emit([0] * 100)

# Small random cases.
for n in range(3, 23):
    emit(int_array(r, n, 0, r.choice([5, 50, 1000])))
for _ in range(15):
    emit(int_array(r, r.randint(1, 12), 0, 10))

# Medium random cases.
for _ in range(8):
    emit(int_array(r, r.randint(23, 99), 0, 1000))

# Large cases at the limit: exponential recursion cannot finish these.
for _ in range(5):
    emit(int_array(r, 100, 0, 1000))
emit([1000] + int_array(r, 98, 0, 10) + [1000])
emit([r.choice([0, 1000]) for _ in range(100)])
emit(list(range(1, 101)))
