import sys
from collections import Counter

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(347)
N, V = 10**5, 10**4


def valid_ks(nums):
    """All k for which the top-k set is unique (no tie across the k-th boundary)."""
    freqs = sorted(Counter(nums).values(), reverse=True)
    return [k for k in range(1, len(freqs) + 1) if k == len(freqs) or freqs[k - 1] > freqs[k]]


def build(freqs, lo=-V, hi=V):
    """An array in which distinct random values occur with the given frequencies, shuffled."""
    values = r.sample(range(lo, hi + 1), len(freqs))
    nums = [v for v, f in zip(values, freqs) for _ in range(f)]
    r.shuffle(nums)
    return nums


def case(nums, k=None):
    ks = valid_ks(nums)
    if k is None or k not in ks:
        k = r.choice(ks) if k is None else min(ks, key=lambda x: abs(x - k))
    emit(nums, k)


def random_case(n, lo, hi):
    case([r.randint(lo, hi) for _ in range(n)])


case([5], 1)
case([-V], 1)
case([V, V, V], 1)
case([1, 2], 2)
case([3, 3, 4], 1)
case([3, 3, 4], 2)
case([4, 1, -1, 2, -1, 2, 3], 2)
case([0, 0, 0, -1, -1, 2], 3)
case([5, 3, 1, 1, 1, 3, 73, 1], 1)
case([5, 3, 1, 1, 1, 3, 73, 1], 2)
case([1, 2, 3, 4, 5], 5)  # all tied: only k = u is unambiguous
case([-V, V, V, -V, -V], 1)
for n in range(2, 16):
    random_case(n, -4, 4)
for _ in range(15):
    random_case(r.randint(16, 400), -30, 30)
for _ in range(8):
    random_case(r.randint(400, 3000), -V, V)
# Large cases at n = 10^5 with many distinct values and both small and large k.
freqs = list(range(5, 305)) + [r.randint(1, 4) for _ in range(19000)]
nums = build(freqs)
case(nums, 1)
case(nums, 300)
big = build([r.randint(1, 9) for _ in range(19000)])[:N]
case(big, len(set(big)) // 2)
case(big, len(set(big)))
case(build([N - 20000] + [1] * 20000), 1)
case([r.randint(-V, V) for _ in range(N)], 3000)
