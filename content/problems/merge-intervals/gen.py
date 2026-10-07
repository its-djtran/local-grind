import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(56)
HI = 10**4


def rand_intervals(n, hi, max_len):
    out = []
    for _ in range(n):
        a = r.randint(0, hi)
        out.append([a, min(hi, a + r.randint(0, max_len))])
    return out


def brute(intervals):
    """Mark covered half-steps on doubled coordinates, then read off maximal runs."""
    hi = max(e for _, e in intervals)
    cover = [False] * (2 * hi + 2)
    for s, e in intervals:
        for x in range(2 * s, 2 * e + 1):
            cover[x] = True
    res, x = [], 0
    while x < len(cover):
        if cover[x]:
            y = x
            while cover[y + 1]:
                y += 1
            res.append([x // 2, y // 2])
            x = y + 1
        else:
            x += 1
    return res


def ref(intervals):
    merged = []
    for s, e in sorted(intervals):
        if merged and s <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], e)
        else:
            merged.append([s, e])
    return merged


def out(intervals):
    assert 1 <= len(intervals) <= 10**4
    assert all(0 <= s <= e <= HI for s, e in intervals)
    if len(intervals) <= 300 and max(e for _, e in intervals) <= 2000:
        assert ref(intervals) == brute(intervals), intervals
    emit(intervals)


out([[1, 1]])
out([[0, 0]])
out([[HI, HI]])
out([[0, HI]])
out([[1, 4], [0, 4]])
out([[1, 4], [0, 0]])
out([[1, 4], [5, 6]])
out([[1, 4], [2, 3]])
out([[1, 10], [2, 3], [4, 5], [11, 12]])
out([[2, 3], [4, 5], [6, 7], [8, 9], [1, 10]])
out([[1, 3], [3, 5], [5, 7], [7, 9]])
out([[5, 5], [5, 5], [5, 5]])
out([[3, 4], [1, 2], [5, 6]])
out([[0, 0], [1, 1], [2, 2], [0, 2]])
out([[1, 4], [0, 2], [3, 5]])
out([[2, 6], [8, 10], [1, 3], [15, 18], [17, 20]])
for n in range(1, 15):
    out(rand_intervals(n, 30, r.choice([0, 2, 5, 10])))
for _ in range(30):
    n = r.randint(10, 300)
    out(rand_intervals(n, r.choice([n, 5 * n, 2000]), r.choice([0, 3, 20, 200])))
for _ in range(6):
    out(rand_intervals(r.randint(1000, 10**4), HI, r.choice([1, 5, 50])))
# Large cases where almost nothing merges, so the output stays large.
pts = [[i, i] for i in range(HI)]
r.shuffle(pts)
out(pts)
pairs = [[2 * i, 2 * i + 1] for i in range(HI // 2)] + [[2 * i, 2 * i] for i in range(HI // 2)]
r.shuffle(pairs)
out(pairs)
for _ in range(3):
    pts = [[x, x] for x in r.sample(range(HI + 1), HI)]
    out(pts)
out(sorted(rand_intervals(HI, HI, 2), reverse=True))
out(rand_intervals(HI, HI, HI))
