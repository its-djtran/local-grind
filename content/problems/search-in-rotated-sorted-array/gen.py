import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(33)
N, V = 5000, 10**4


def rotated(n, k=None, lo=-V, hi=V):
    vals = sorted(r.sample(range(lo, hi + 1), n))
    if k is None:
        k = r.randrange(n)
    return vals[k:] + vals[:k]


def missing(nums, lo=-V, hi=V):
    present = set(nums)
    while True:
        t = r.randint(max(-V, lo), min(V, hi))
        if t not in present:
            return t


emit([1], 1)
emit([1], 2)
emit([-V], -V)
emit([V], -V)
emit([1, 3], 3)
emit([3, 1], 1)
emit([3, 1], 3)
emit([3, 1], 2)
emit([1, 3], 0)
emit([5, 1, 3], 5)
emit([5, 1, 3], 3)
emit([4, 5, 6, 7, 0, 1, 2], 7)
emit([4, 5, 6, 7, 0, 1, 2], 4)
emit([4, 5, 6, 7, 0, 1, 2], 2)
emit([4, 5, 6, 7, 8, 1, 2, 3], 8)
emit([1, 2, 3, 4, 5, 6], 6)
emit([6, 1, 2, 3, 4, 5], 6)
emit([2, 3, 4, 5, 6, 1], 1)
for _ in range(25):
    n = r.randint(1, 12)
    nums = rotated(n, None, -30, 30)
    if r.random() < 0.65:
        emit(nums, r.choice(nums))
    else:
        emit(nums, missing(nums, -35, 35))
for _ in range(15):
    nums = rotated(r.randint(13, 500))
    emit(nums, r.choice(nums) if r.random() < 0.6 else missing(nums))
# Large arrays: the target at the pivot, at both ends, absent, beyond both extremes.
for k in (0, 1, N // 3, N - 1, r.randrange(N)):
    nums = rotated(N, k)
    emit(nums, nums[0])
    emit(nums, nums[-1])
    emit(nums, min(nums))
    emit(nums, missing(nums))
nums = list(range(1, N + 1))
emit(nums, 0)
emit(nums, N + 1)
