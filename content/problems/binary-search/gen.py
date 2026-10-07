import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(704)
MAX_N = 10**4
LO, HI = -9999, 9999  # values and target lie strictly between -10^4 and 10^4


def check_and_emit(nums, target):
    assert 1 <= len(nums) <= MAX_N and all(LO <= x <= HI for x in nums) and LO <= target <= HI
    assert all(nums[i] < nums[i + 1] for i in range(len(nums) - 1))
    emit(nums, target)


def sorted_distinct(n):
    return sorted(r.sample(range(LO, HI + 1), n))


def absent(nums):
    present = set(nums)
    choices = [v for v in (r.randint(LO, HI) for _ in range(50)) if v not in present]
    return choices[0] if choices else None


check_and_emit([5], 5)
check_and_emit([5], -5)
check_and_emit([5], 6)
check_and_emit([LO], LO)
check_and_emit([HI], LO)
check_and_emit([1, 2], 1)
check_and_emit([1, 2], 2)
check_and_emit([1, 3], 2)
check_and_emit([1, 3], 0)
check_and_emit([1, 3], 4)
check_and_emit([-1, 0, 3, 5, 9, 12], -1)
check_and_emit([-1, 0, 3, 5, 9, 12], 12)
check_and_emit([-1, 0, 3, 5, 9, 12], 13)
check_and_emit([-1, 0, 3, 5, 9, 12], -2)
for _ in range(15):
    nums = sorted_distinct(r.randint(1, 30))
    t = r.choice(nums) if r.random() < 0.6 else absent(nums)
    if t is None:
        t = nums[0]
    check_and_emit(nums, t)
for _ in range(6):
    nums = sorted_distinct(r.randint(30, 2000))
    t = r.choice(nums) if r.random() < 0.5 else absent(nums)
    if t is None:
        t = nums[0]
    check_and_emit(nums, t)
# Maximal arrays. Targets near the right end or absent make a linear scan read the whole array.
for k in range(56):
    nums = sorted_distinct(MAX_N)
    kind = k % 4
    if kind == 0:
        t = nums[-1]
    elif kind == 1:
        t = nums[-1 - r.randint(0, 50)]
    elif kind == 2:
        t = HI if nums[-1] != HI else HI - 1
        if t in nums:
            t = absent(nums) or nums[-1]
    else:
        t = absent(nums) or nums[-1]
    check_and_emit(nums, t)
check_and_emit(list(range(LO, LO + MAX_N)), LO + MAX_N - 1)
check_and_emit(list(range(HI - MAX_N + 1, HI + 1)), HI)
check_and_emit(list(range(LO, LO + 2 * MAX_N, 2)), LO + 1)
