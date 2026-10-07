import sys
from collections import Counter

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(169)
MAX_N = 5 * 10**4
LIM = 10**9


def check_and_emit(nums):
    n = len(nums)
    assert 1 <= n <= MAX_N and all(-LIM <= x <= LIM for x in nums)
    counts = Counter(nums)
    best, c = counts.most_common(1)[0]
    assert c > n // 2
    emit(nums)


def case(n, lo, hi, extra=0, order="shuffle"):
    """A majority value appearing n // 2 + 1 + extra times; the rest drawn from [lo, hi]."""
    maj = r.randint(lo, hi)
    k = min(n, n // 2 + 1 + extra)
    others = [r.randint(lo, hi) for _ in range(n - k)]
    others = [x if x != maj else (x + 1 if x < hi else x - 1) for x in others]
    if order == "shuffle":
        nums = [maj] * k + others
        r.shuffle(nums)
    elif order == "tail":
        nums = others + [maj] * k  # majority hidden behind every other value
    else:
        nums = [maj] * k + others
    check_and_emit(nums)


check_and_emit([1])
check_and_emit([-LIM])
check_and_emit([LIM, LIM])
check_and_emit([5, 5, 7])
check_and_emit([7, 5, 5])
check_and_emit([1, 2, 1, 2, 1])
check_and_emit([2, 1, 2, 1, 1])
check_and_emit([0, 0, 0, 0])
check_and_emit([-1, -1, 3, 3, -1])
check_and_emit([LIM, -LIM, LIM])
check_and_emit([6, 5, 5])
for n in range(1, 16):
    case(n, -3, 3)
for _ in range(20):
    case(r.randint(16, 500), -50, 50, extra=r.randint(0, 5))
for _ in range(10):
    case(r.randint(500, 5000), -LIM, LIM)
for _ in range(3):
    case(r.randint(5000, MAX_N), -10**4, 10**4, extra=r.randint(0, 1000))
# Maximal arrays: the first half holds non-majority values, defeating early exits.
case(MAX_N, -LIM, LIM, order="tail")
case(MAX_N, -10**5, 10**5, order="tail")
case(MAX_N - 1, -10**5, 10**5, order="tail")
case(MAX_N, -10**4, 10**4)
case(MAX_N, -10**4, 10**4, order="head")
check_and_emit([-7] * MAX_N)
