import sys
from itertools import combinations

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(435)
LO, HI = -5 * 10**4, 5 * 10**4


def rand_intervals(n, lo, hi, max_len):
    out = []
    for _ in range(n):
        a = r.randint(lo, hi - 1)
        out.append([a, min(hi, a + r.randint(1, max_len))])
    return out


def compatible(ivs):
    ivs = sorted(ivs)
    return all(x[1] <= y[0] for x, y in zip(ivs, ivs[1:]))


def brute(ivs):
    n = len(ivs)
    for keep in range(n, 0, -1):
        if any(compatible(c) for c in combinations(ivs, keep)):
            return n - keep
    return n


def dp(ivs):
    ivs = sorted(ivs)
    keep = [1] * len(ivs)
    for i in range(len(ivs)):
        for j in range(i):
            if ivs[j][1] <= ivs[i][0]:
                keep[i] = max(keep[i], keep[j] + 1)
    return len(ivs) - max(keep)


def ref(ivs):
    removed, last = 0, float("-inf")
    for s, e in sorted(ivs, key=lambda iv: iv[1]):
        if s >= last:
            last = e
        else:
            removed += 1
    return removed


def out(ivs):
    assert 1 <= len(ivs) <= 10**5
    assert all(LO <= s < e <= HI for s, e in ivs)
    if len(ivs) <= 12:
        assert ref(ivs) == brute(ivs), ivs
    elif len(ivs) <= 400:
        assert ref(ivs) == dp(ivs), ivs
    emit(ivs)


out([[1, 2]])
out([[LO, HI]])
out([[LO, LO + 1], [HI - 1, HI]])
out([[1, 2], [2, 3]])
out([[1, 3], [2, 4]])
out([[1, 100], [11, 22], [1, 11], [2, 12]])
out([[0, 2], [1, 3], [2, 4], [3, 5], [4, 6]])
out([[1, 10], [2, 3], [3, 4], [4, 5]])
out([[-3, -1], [-2, 0], [-1, 1]])
out([[5, 6]] * 7)
out([[i, i + 1] for i in range(-10, 10)])
out([[0, 50], [1, 2], [2, 3], [3, 4], [49, 50]])
for n in range(2, 13):
    out(rand_intervals(n, 0, 20, r.choice([1, 3, 8])))
for _ in range(30):
    n = r.randint(13, 400)
    out(rand_intervals(n, r.choice([-n, LO]), r.choice([n, HI]), r.choice([1, 5, 50, 5000])))
for _ in range(6):
    out(rand_intervals(r.randint(2000, 10**4), LO, HI, r.choice([10, 100, 1000])))
# Large cases at the limits.
out(rand_intervals(10**5, LO, HI, 100))
out(rand_intervals(5 * 10**4, LO, HI, 10**4))
chain = [[i, i + 1] for i in range(LO, HI)]
r.shuffle(chain)
out(chain[: 3 * 10**4])
out([[LO + i, HI - i] for i in range(2 * 10**4)])
