import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(252)
HI = 10**6


def disjoint(n, hi=HI, touch_prob=0.3):
    """n non-overlapping meetings (some back-to-back), shuffled."""
    pts = sorted(r.sample(range(hi + 1), 2 * n))
    out = [[pts[2 * k], pts[2 * k + 1]] for k in range(n)]
    for k in range(1, n):
        if r.random() < touch_prob:
            out[k][0] = out[k - 1][1]
    r.shuffle(out)
    return out


def with_clash(n, hi=HI):
    """A disjoint schedule with one meeting stretched to overlap a neighbour."""
    ivs = sorted(disjoint(n, hi))
    k = r.randrange(n - 1)
    ivs[k][1] = r.randint(ivs[k + 1][0] + 1, ivs[k + 1][1])
    r.shuffle(ivs)
    return ivs


def brute(ivs):
    return not any(
        a[0] < b[1] and b[0] < a[1] for i, a in enumerate(ivs) for b in ivs[i + 1:]
    )


def out(ivs):
    assert 0 <= len(ivs) <= 10**4
    assert all(0 <= s < e <= HI for s, e in ivs)
    expect = brute(ivs) if len(ivs) <= 500 else None
    ref = all(b[0] >= a[1] for a, b in zip(sorted(ivs), sorted(ivs)[1:]))
    assert expect is None or expect == ref, ivs
    emit(ivs)


out([])
out([[0, 1]])
out([[0, HI]])
out([[1, 5], [5, 10]])
out([[5, 10], [1, 5]])
out([[1, 5], [4, 10]])
out([[1, 10], [2, 3]])
out([[1, 5], [1, 5]])
out([[1, 5], [1, 2]])
out([[0, 1], [1, 2], [2, 3], [3, 4]])
out([[3, 4], [2, 3], [1, 2], [0, 1], [1, 3]])
out([[0, HI - 1], [HI - 1, HI]])
for n in range(2, 12):
    out(disjoint(n, 40))
    out(with_clash(n, 40))
for _ in range(15):
    n = r.randint(12, 500)
    out(disjoint(n, r.choice([3 * n, HI])))
    out(with_clash(n, r.choice([3 * n, HI])))
# Large valid schedules: a quadratic checker must examine every pair.
for _ in range(5):
    out(disjoint(10**4))
out([[2 * i, 2 * i + 2] for i in range(10**4)][::-1])
# Large schedules whose only clash is between the two latest meetings.
for _ in range(2):
    ivs = sorted(disjoint(10**4, HI, 0.0))
    ivs[-2][1] = ivs[-1][0] + 1
    out(ivs)
