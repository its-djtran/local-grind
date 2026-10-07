import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(1)
LIMIT = 10**9


def unique_pair_count(nums, target):
    seen, pairs = {}, 0
    for x in nums:
        pairs += seen.get(target - x, 0)
        seen[x] = seen.get(x, 0) + 1
    return pairs


def case(n, lo, hi):
    while True:
        nums = [r.randint(lo, hi) for _ in range(n)]
        i, j = r.sample(range(n), 2)
        target = nums[i] + nums[j]
        if abs(target) <= LIMIT and unique_pair_count(nums, target) == 1:
            return nums, target


emit([1, 2], 3)
emit([-1, -2], -3)
emit([0, 4, 3, 0], 0)
emit([-3, 4, 3, 90], 0)
emit([LIMIT, -LIMIT], 0)
emit([5, 75, 25], 100)
emit([2, 5, 5, 11], 10)
for n in range(2, 12):
    emit(*case(n, -10, 10))
for _ in range(25):
    emit(*case(r.randint(10, 200), -1000, 1000))
for _ in range(10):
    emit(*case(r.randint(1000, 10**4), -LIMIT // 2, LIMIT // 2))
# Answer at the very end of a maximal array defeats early-exit shortcuts.
nums = list(range(1, 10**4 - 1)) + [10**8, 10**8 + 1]
emit(nums, 2 * 10**8 + 1)
nums = [x * 2 for x in range(10**4)]
emit(nums, nums[-1] + nums[-2])
