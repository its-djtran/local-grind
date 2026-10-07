import sys

sys.path.insert(0, "../../lib")
from lg_gen import distinct_ints, emit, rng

r = rng(46)


def case(nums):
    assert 1 <= len(nums) <= 6 and len(set(nums)) == len(nums)
    assert all(-10 <= x <= 10 for x in nums)
    emit(nums)


# Edge cases.
case([0])
case([-10])
case([10])
case([-10, 10])
case([5, -5])
case([3, 2, 1])
case([-1, 0, 1])
case([1, 2, 3, 4])
case([-10, -9, -8, -7, -6, -5])
case([10, 9, 8, 7, 6, 5])
case([0, 1, 2, 3, 4, 5])
case([-10, 10, 0, -1, 1, 7])

# Random cases of every size; most at the maximum size, where the output has 720 lists.
for n in range(1, 7):
    for _ in range(4):
        case(distinct_ints(r, n, -10, 10))
for _ in range(30):
    case(distinct_ints(r, 6, -10, 10))
