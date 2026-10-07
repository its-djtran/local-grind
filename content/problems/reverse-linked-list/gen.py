import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(206)
emit([1])
emit([0, 0])
emit([-5000, 5000])
emit([5000, -5000, 5000])
emit([7] * 10)
emit(list(range(10)))
for n in range(3, 20):
    emit(int_array(r, n, -100, 100))
for _ in range(30):
    emit(int_array(r, r.randint(20, 1000), -5000, 5000))
for _ in range(5):
    emit(int_array(r, 5000, -5000, 5000))
emit(list(range(-2500, 2500)))
