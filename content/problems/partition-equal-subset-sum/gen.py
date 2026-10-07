import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(416)


def splittable(n, lo, hi):
    """Random array that is guaranteed to be partitionable (built from two equal-sum halves)."""
    while True:
        a = int_array(r, n // 2, lo, hi)
        b = int_array(r, n - n // 2 - 1, lo, hi)
        diff = sum(a) - sum(b)
        if lo <= diff <= hi:
            out = a + b + [diff]
            r.shuffle(out)
            return out


def even_odd_half(n, lo, hi):
    """Only even values but total/2 odd: impossible, yet the total is even."""
    while True:
        a = [2 * x for x in int_array(r, n, (lo + 1) // 2, hi // 2)]
        if sum(a) % 4 == 2:
            return a


# Edge cases.
emit([1])
emit([100])
emit([1, 1])
emit([1, 2])
emit([100, 100])
emit([1, 2, 5])
emit([2, 2, 3, 5])
emit([3, 3, 3, 4, 5])
emit([1, 1, 1, 1])
emit([1, 2, 3, 4, 5, 6, 7])
emit([2, 2, 1, 1])
emit([1] * 200)
emit([1] * 199)
emit([100] * 200)
emit([100] * 199 + [1])

# Small random cases.
for _ in range(25):
    emit(int_array(r, r.randint(1, 12), 1, r.choice([5, 20, 100])))
for _ in range(10):
    emit(splittable(r.randint(2, 12), 1, 20))

# Medium random cases.
for _ in range(8):
    emit(int_array(r, r.randint(13, 150), 1, 100))
for _ in range(4):
    emit(even_odd_half(r.randint(13, 150), 2, 100))

# Large cases at the limits: exhaustive search dies on the impossible even-total ones.
for _ in range(3):
    emit(even_odd_half(200, 2, 100))
emit([100] + [1] * 98)                        # one element larger than everything else combined
emit([99] * 199 + [1])
emit(even_odd_half(200, 50, 100))
for _ in range(3):
    emit(splittable(200, 1, 100))
emit(int_array(r, 200, 1, 100))
emit(int_array(r, 200, 90, 100))
