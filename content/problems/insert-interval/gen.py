import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(57)
HI = 10**5


def sorted_disjoint(n, lo=0, hi=HI, point_prob=0.1):
    """n sorted intervals in [lo, hi] with a gap between consecutive ones."""
    pts = sorted(r.sample(range(lo, hi + 1), 2 * n))
    out = []
    for k in range(n):
        a, b = pts[2 * k], pts[2 * k + 1]
        if r.random() < point_prob:
            b = a
        out.append([a, b])
    return out


def check(intervals):
    for a, b in intervals:
        assert 0 <= a <= b <= HI
    for x, y in zip(intervals, intervals[1:]):
        assert x[1] < y[0]


def new_interval(lo, hi):
    a = r.randint(lo, hi)
    b = min(hi, a + r.choice([0, 1, r.randint(0, max(1, (hi - lo) // 10)), r.randint(0, hi - lo)]))
    return [a, b]


def brute(intervals, new):
    """Covered-point reasoning on doubled coordinates (handles touching and points)."""
    merged = []
    for s, e in sorted(intervals + [new]):
        if merged and s <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], e)
        else:
            merged.append([s, e])
    return merged


def ref(intervals, new):
    s, e = new
    res, i = [], 0
    while i < len(intervals) and intervals[i][1] < s:
        res.append(intervals[i])
        i += 1
    while i < len(intervals) and intervals[i][0] <= e:
        s, e = min(s, intervals[i][0]), max(e, intervals[i][1])
        i += 1
    return res + [[s, e]] + intervals[i:]


def out(intervals, new):
    check(intervals)
    assert 0 <= new[0] <= new[1] <= HI
    if len(intervals) <= 300:
        assert ref(intervals, new) == brute(intervals, new)
    emit(intervals, new)


out([], [5, 7])
out([], [0, 0])
out([[1, 5]], [2, 3])
out([[1, 5]], [6, 8])
out([[1, 5]], [0, 0])
out([[1, 5]], [0, 1])
out([[1, 5]], [5, 9])
out([[3, 5]], [1, 2])
out([[1, 2], [3, 5]], [2, 3])
out([[1, 2], [4, 5]], [3, 3])
out([[0, 0], [2, 2], [4, 4]], [1, 3])
out([[1, 3], [6, 9]], [0, HI])
out([[1, 3], [6, 9]], [10, 10])
out([[0, 0]], [0, 0])
out([[HI, HI]], [0, HI])
out([[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], [17, 20])
out([[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], [0, 0])
out([[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], [10, 12])
for n in range(1, 12):
    ivs = sorted_disjoint(n, 0, 40, 0.2)
    out(ivs, new_interval(0, 40))
for _ in range(30):
    n = r.randint(1, 200)
    hi = r.choice([3 * n, 10 * n, HI])
    ivs = sorted_disjoint(n, 0, hi)
    out(ivs, new_interval(0, hi))
for _ in range(5):
    ivs = sorted_disjoint(r.randint(1000, 5000))
    out(ivs, new_interval(0, HI))
# Large cases: the new interval lands at the front, the back, the middle, or swallows everything.
big = sorted_disjoint(10**4, 0, HI, 0.05)
out(big, [big[-1][1] + 1, HI] if big[-1][1] < HI else [HI, HI])
out(big, [0, 0])
out(big, [big[5000][0], big[5000][1]])
out(big, [0, HI])
for _ in range(6):
    out(sorted_disjoint(10**4, 0, HI, 0.05), new_interval(0, HI))
