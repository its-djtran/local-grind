import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(268)
MAX = 10**4


def case(n, missing, order="shuffle"):
    nums = [v for v in range(n + 1) if v != missing]
    if order == "shuffle":
        r.shuffle(nums)
    elif order == "desc":
        nums.reverse()
    assert len(nums) == n and len(set(nums)) == n and all(0 <= v <= n for v in nums)
    emit(nums)


case(1, 0)
case(1, 1)
case(2, 0)
case(2, 1)
case(2, 2, "asc")
case(5, 5, "asc")
case(5, 0, "asc")
case(6, 3, "desc")
for n in range(3, 15):
    case(n, r.randint(0, n))
for _ in range(25):
    n = r.randint(15, 1000)
    case(n, r.randint(0, n))
for _ in range(5):
    n = r.randint(1000, MAX)
    case(n, r.randint(0, n))
# Maximal arrays where the gap is at or near the top of the range.
case(MAX, MAX)
case(MAX, MAX, "asc")
case(MAX, MAX - 1)
case(MAX, MAX, "desc")
for _ in range(6):
    case(MAX, r.randint(MAX - 100, MAX))
case(MAX, 0)
case(MAX, MAX // 2)
