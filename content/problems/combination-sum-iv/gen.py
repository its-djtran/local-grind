import sys

sys.path.insert(0, "../../lib")
from lg_gen import distinct_ints, emit, rng

r = rng(377)
INT_MAX = 2**31 - 1


def table(nums, limit):
    dp = [1] + [0] * limit
    for t in range(1, limit + 1):
        dp[t] = sum(dp[t - x] for x in nums if x <= t)
    return dp


def answer(nums, target):
    return table(nums, target)[target]


def biggest_target(nums, cap=1000):
    """Largest target <= cap whose answer still fits in 32 bits (and is nonzero if possible)."""
    dp = table(nums, cap)
    fits = [t for t in range(1, cap + 1) if 0 < dp[t] <= INT_MAX]
    return fits[-1] if fits else 1


def check(nums, target):
    assert 1 <= len(nums) <= 200 and len(set(nums)) == len(nums)
    assert all(1 <= x <= 1000 for x in nums) and 1 <= target <= 1000
    assert answer(nums, target) <= INT_MAX
    emit(nums, target)


# Edge cases.
check([1], 1)
check([1], 1000)
check([1000], 1000)
check([2], 1)
check([2], 1000)
check([3, 1, 2], 4)
check([4, 2, 1], 32)
check([1, 2], 44)
check([5, 10], 3)
check([999, 1000], 1000)
check([1, 50], 200)
check([10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 250)
check(list(range(801, 1001)), 1000)

# Small random cases.
for _ in range(30):
    while True:
        nums = distinct_ints(r, r.randint(1, 5), 1, 20)
        target = r.randint(1, 60)
        if answer(nums, target) <= INT_MAX:
            break
    check(nums, target)

# Medium random cases with targets pushed close to the 32-bit limit.
for _ in range(10):
    lo = r.choice([1, 5, 20, 60])
    nums = distinct_ints(r, r.randint(2, 30), lo, min(1000, lo + 300))
    check(nums, r.randint(1, biggest_target(nums)))

# Large cases: answers near 2^31 make plain recursion hopeless.
for lo, n in [(1, 200), (40, 200), (100, 200), (150, 200), (200, 200), (3, 3), (7, 12)]:
    nums = distinct_ints(r, n, lo, 1000) if n == 200 else distinct_ints(r, n, lo, lo + 20)
    check(nums, biggest_target(nums))
check([1, 2, 3], biggest_target([1, 2, 3]))
check(list(range(1, 201)), biggest_target(list(range(1, 201))))
