import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(152)
N = 2 * 10**4
LIMIT = 2**31 - 1


def valid(nums):
    """Every subarray product fits in 32 bits iff every zero-free stretch does (in absolute value)."""
    p = 1
    for x in nums:
        if x == 0:
            p = 1
            continue
        p *= abs(x)
        if p > LIMIT:
            return False
    return True


def small_random(n, lo, hi):
    while True:
        nums = [r.randint(lo, hi) for _ in range(n)]
        if valid(nums):
            return nums


def sparse(n, big, zero_prob):
    """Mostly +-1 (and some zeros), with up to `big` values of magnitude 2..10."""
    while True:
        nums = [0 if r.random() < zero_prob else r.choice((1, -1)) for _ in range(n)]
        for i in r.sample(range(n), min(n, big)):
            nums[i] = r.choice((-1, 1)) * r.randint(2, 10)
        if valid(nums):
            return nums
        big = max(0, big - 1)


emit([0])
emit([-2])
emit([10])
emit([-10])
emit([-2, 3, -4])
emit([3, -1, 4])
emit([-2, -3])
emit([0, 2])
emit([2, 0])
emit([-1, 0, -1])
emit([0, 0, 0])
emit([-2, 0, -3, 0, -4])
emit([2, -5, -2, -4, 3])
emit([-1, -2, -3, 0])
emit([1, -2, 3, -4, 5, -6, 7, 0, 2])
emit([-10] * 9)
emit([10] * 9)
for n in range(2, 10):
    for _ in range(3):
        emit(small_random(n, -10, 10))
for _ in range(10):
    emit(small_random(r.randint(10, 25), -3, 3))
for _ in range(10):
    emit(sparse(r.randint(25, 2000), r.randint(0, 30), r.choice([0.0, 0.01, 0.1])))
# Large cases at the length limit: several stretches, one long zero-free stretch, all +-1.
emit(sparse(N, 40, 0.002))
emit(sparse(N, 25, 0.0))
emit(sparse(N, 60, 0.01))
emit([r.choice((1, -1)) for _ in range(N)])
emit([-1] * (N - 1) + [-2])
emit([0] * (N // 2) + [-2, 5] + [0] * (N // 2 - 2))
