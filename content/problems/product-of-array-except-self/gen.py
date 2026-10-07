import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(238)
N = 10**5
LO, HI = -(2**31), 2**31 - 1


def fits(x):
    return LO <= x <= HI


def valid(nums):
    """Every prefix and suffix product (and therefore every answer) fits in 32 bits."""
    for seq in (nums, nums[::-1]):
        p = 1
        for x in seq:
            p *= x
            if not fits(p):
                return False
    # Answers can combine a prefix and a suffix around a zero; keep those in range too.
    zeros = [i for i, x in enumerate(nums) if x == 0]
    if len(zeros) == 1:
        p = 1
        for i, x in enumerate(nums):
            if i != zeros[0]:
                p *= x
        if not fits(p):
            return False
    return True


def case(n, big, zeros):
    """Mostly +-1, with `big` random values in [-30, 30] and `zeros` zeros, at random spots."""
    while True:
        nums = [r.choice((1, -1)) for _ in range(n)]
        spots = r.sample(range(n), min(n, big + zeros))
        for k, i in enumerate(spots):
            nums[i] = 0 if k < zeros else r.choice([v for v in range(-30, 31) if v not in (0,)])
        if valid(nums):
            return nums
        big = max(0, big - 1)


def small_random(n):
    while True:
        nums = [r.randint(-30, 30) for _ in range(n)]
        if valid(nums):
            return nums


emit([1, 1])
emit([0, 0])
emit([0, 5])
emit([5, 0])
emit([-30, 30])
emit([2, 3])
emit([1, 0, 0, 1])
emit([0, 0, 0, 0])
emit([-1, -1, -1])
emit([30, 30, 30, 30, 30, 30])
emit([-30, 30, -30, 30, -30, 30])
emit([2] * 30)
emit([-2] * 31)
emit([7, 0, 3, 2, -4])
emit([3, 0, 2, 0, 5])
for n in range(2, 8):
    for _ in range(3):
        emit(small_random(n))
for _ in range(15):
    n = r.randint(8, 60)
    emit(case(n, r.randint(0, 8), r.choice([0, 0, 0, 1, 2])))
for _ in range(10):
    n = r.randint(60, 3000)
    emit(case(n, r.randint(0, 12), r.choice([0, 1, 2, 3])))
# Large cases at the length limit: no zero, one zero, two zeros, all ones.
emit(case(N, 12, 0))
emit(case(N, 12, 1))
emit(case(N, 10, 2))
emit([1] * N)
emit([-1] * (N - 1) + [30])
