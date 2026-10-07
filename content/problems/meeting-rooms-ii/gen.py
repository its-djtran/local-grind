import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(253)
HI = 10**6


def rand_meetings(n, hi, max_len):
    out = []
    for _ in range(n):
        s = r.randint(0, hi - 1)
        out.append([s, min(hi, s + r.randint(1, max_len))])
    return out


def brute(ivs):
    """Sweep every integer instant."""
    hi = max(e for _, e in ivs)
    delta = [0] * (hi + 2)
    for s, e in ivs:
        delta[s] += 1
        delta[e] -= 1
    best = cur = 0
    for d in delta:
        cur += d
        best = max(best, cur)
    return best


def ref(ivs):
    starts, ends = sorted(s for s, _ in ivs), sorted(e for _, e in ivs)
    rooms = j = 0
    for s in starts:
        if ends[j] <= s:
            j += 1
        else:
            rooms += 1
    return rooms


def out(ivs):
    assert 1 <= len(ivs) <= 10**4
    assert all(0 <= s < e <= HI for s, e in ivs)
    assert ref(ivs) == brute(ivs), ivs
    emit(ivs)


out([[0, 1]])
out([[0, HI]])
out([[1, 5], [5, 10]])
out([[1, 5], [4, 10]])
out([[1, 5], [1, 5], [1, 5]])
out([[1, 10], [2, 3], [3, 4], [4, 5]])
out([[0, 1], [1, 2], [2, 3], [3, 4]])
out([[9, 10], [4, 9], [4, 17]])
out([[2, 11], [6, 16], [11, 16]])
out([[1, 8], [6, 20], [9, 16], [13, 17]])
out([[i, 100 - i] for i in range(50)])
out([[0, HI - 1], [HI - 1, HI], [0, HI]])
for n in range(2, 15):
    out(rand_meetings(n, 30, r.choice([2, 5, 15])))
for _ in range(30):
    n = r.randint(15, 1000)
    out(rand_meetings(n, r.choice([n, 10 * n, HI]), r.choice([3, 50, 1000, HI])))
# Large cases.
out(rand_meetings(10**4, HI, 1000))
out(rand_meetings(10**4, HI, 10**5))
out(rand_meetings(10**4, HI, HI))
out([[i, i + 1] for i in range(10**4)][::-1])
out([[i, HI - i] for i in range(10**4)])
out(rand_meetings(10**4, 1000, 5))
