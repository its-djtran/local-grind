import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(55)
N, V = 10**4, 10**5


def trap(n, t, tail_hi):
    """Every index before t can jump exactly to t, which holds 0, so nothing beyond t is reachable."""
    return [t - i for i in range(t)] + [0] + int_array(r, n - t - 1, 0, tail_hi)


# Edge cases.
emit([0])
emit([V])
emit([1, 0])
emit([0, 1])
emit([0, 0])
emit([2, 0, 0])
emit([1, 0, 1])
emit([1, 1, 0, 1])
emit([2, 5, 0, 0])
emit([1, 2, 3])
emit([3, 0, 8, 2, 0, 0, 1])
emit([4, 0, 0, 0, 0])
emit([4, 0, 0, 0, 0, 1])

# Small random cases (values small enough that zeros often block the way).
for _ in range(30):
    n = r.randint(1, 15)
    emit([r.choice([0, 0, 1, 1, 2, 3, 4]) for _ in range(n)])

# Medium random cases.
for _ in range(10):
    n = r.randint(16, 2000)
    hi = r.choice([2, 3, 10])
    emit(int_array(r, n, 0, hi))

# Large cases at the limits.
emit([V] * N)
emit([1] * N)
emit([1] * (N - 2) + [0, 0])                 # stuck one short of the end
emit([0] + [V] * (N - 1))
emit(trap(N, N - 2, V))                       # the quadratic DP scans every range
emit(trap(N, N - 2, 5))
emit(trap(N, N - 100, V))
emit(trap(N, N // 2, V))
emit([N - 1 - i for i in range(N)])           # true, each index reaches the end exactly
emit(int_array(r, N, 1, 3))                   # true: no zeros
emit([r.randint(1, 4) if i % 50 else 0 for i in range(N)])
