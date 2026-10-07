import sys

sys.path.insert(0, "../../lib")
from lg_gen import distinct_ints, emit, int_array, rng

r = rng(217)
N, LIM = 10**5, 10**9


def with_one_repeat(n, lo, hi):
    """n values, all distinct except one value that appears exactly twice."""
    nums = distinct_ints(r, n - 1, lo, hi)
    nums.append(r.choice(nums))
    r.shuffle(nums)
    return nums


emit([0])
emit([LIM])
emit([-LIM])
emit([1, 1])
emit([1, 2])
emit([-LIM, LIM])
emit([LIM, LIM])
emit([-1, 1, -1])
emit([0, -0, 1])
emit([7] * 10)
emit(list(range(10)))
emit(list(range(10)) + [0])  # repeat at the very ends
for n in range(2, 16):
    emit(int_array(r, n, -n, n))  # small range: duplicates likely, but not certain
for _ in range(10):
    emit(distinct_ints(r, r.randint(2, 50), -100, 100))
for _ in range(10):
    emit(with_one_repeat(r.randint(2, 50), -100, 100))
for _ in range(6):
    emit(distinct_ints(r, r.randint(100, 2000), -LIM, LIM))
for _ in range(6):
    emit(with_one_repeat(r.randint(100, 2000), -LIM, LIM))
# Large: no duplicates at all (no early exit), and a single repeat placed late.
emit(distinct_ints(r, N, -LIM, LIM))
emit(list(range(-N // 2, N // 2)))
emit(distinct_ints(r, N, -N, N))
big = distinct_ints(r, N - 1, -10**6, 10**6)
emit(big + [big[0]])
emit(with_one_repeat(N, -10**6, 10**6))
