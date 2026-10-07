import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(128)
N, V = 10**5, 10**9


def shuffled(xs):
    xs = list(xs)
    r.shuffle(xs)
    return xs


def runs(lengths, spread=V, max_gap=10**5):
    """Disjoint runs of the given lengths separated by random gaps (>= 2, so runs never merge)."""
    gaps = [r.randint(2, max_gap) for _ in lengths]
    total = sum(lengths) + sum(gaps)
    x = r.randint(-spread, spread - total)
    out = []
    for L, g in zip(lengths, gaps):
        out.extend(range(x, x + L))
        x += L + g
    return shuffled(out)


emit([])
emit([0])
emit([V])
emit([-V])
emit([5, 5, 5])
emit([1, 2])
emit([2, 1])
emit([1, 3])
emit([-V, V])
emit([V - 1, V, -V, -V + 1, -V + 2])
emit([-2, -1, 0, 1, 2])
emit([9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6])
emit([1, 2, 0, 1])
emit([10, 5, 12, 3, 55, 30, 4, 11, 2])
emit([0, -1, 1, 1, 2, 2, 3, 100, 101])
for n in range(1, 16):
    emit(int_array(r, n, -10, 10))
for _ in range(10):
    emit(runs([r.randint(1, 8) for _ in range(r.randint(1, 6))], 100, 4))
for _ in range(10):
    xs = runs([r.randint(1, 200) for _ in range(r.randint(1, 15))])
    xs += [r.choice(xs) for _ in range(len(xs) // 4)]  # duplicates
    emit(shuffled(xs))
for _ in range(5):
    emit(int_array(r, r.randint(100, 3000), -V, V))
# Large cases: a single run of length n (defeats re-walking), a few long runs, many short runs,
# heavy duplicates, and random spread-out values.
emit(shuffled(range(-N // 2, N - N // 2)))
emit(shuffled(range(V - N // 4 + 1, V + 1)))
emit(runs([N // 2, N // 4, N // 4], 10**6))
half = list(range(N // 2))
emit(shuffled(half + half))
emit(int_array(r, N // 2, -V, V))
emit(runs([r.randint(1, 50) for _ in range(3500)], 10**6, 50))
