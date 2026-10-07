import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(371)
L = 1000

fixed = [
    (0, 0), (0, 1), (1, 0), (-1, 1), (1, -1), (-1, -1), (0, -1),
    (L, L), (-L, -L), (L, -L), (-L, L), (L, 0), (0, -L),
    (-1, L), (L, -1), (-L, 1), (999, 1), (-999, -1), (512, 512), (-512, -512),
    (255, 1), (-256, 256), (7, -8), (-7, 8),
]
for a, b in fixed:
    emit(a, b)
for _ in range(30):
    emit(r.randint(-20, 20), r.randint(-20, 20))
for _ in range(30):
    emit(r.randint(-L, L), r.randint(-L, L))
# Mixed signs exercise long carry chains through the sign bits.
for _ in range(10):
    a = r.randint(1, L)
    emit(a, -r.randint(1, L))
