import sys

sys.path.insert(0, "../../lib")
from lg_gen import distinct_ints, emit, rng

r = rng(78)


def case(nums):
    assert 1 <= len(nums) <= 10 and len(set(nums)) == len(nums)
    assert all(-10 <= x <= 10 for x in nums)
    emit(nums)


# Edge cases.
case([1])
case([-10])
case([10])
case([0, 1])
case([-1, 1])
case([10, -10])
case([3, 2, 1])
case([0, -1, 1, -2, 2])
case(list(range(10)))
case(list(range(-10, 0)))
case(list(range(10, 0, -1)))
case([-10, 10, 0, 5, -5, 1, -1, 9, -9, 3])

# Random cases of every size; the largest ones produce 1024 subsets.
for n in range(1, 11):
    for _ in range(4):
        case(distinct_ints(r, n, -10, 10))
for _ in range(20):
    case(distinct_ints(r, 10, -10, 10))
