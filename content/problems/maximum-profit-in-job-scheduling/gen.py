import bisect
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(1235)
TMAX = 10**9


def jobs(n, tmax, max_len, pmax=10**4):
    st, en, pr = [], [], []
    for _ in range(n):
        s = r.randint(1, tmax - 1)
        st.append(s)
        en.append(min(tmax, s + r.randint(1, max_len)))
        pr.append(r.randint(1, pmax))
    return st, en, pr


def ref(st, en, pr):
    js = sorted(zip(en, st, pr))
    ends = [e for e, _, _ in js]
    dp = [0] * (len(js) + 1)
    for i, (e, s, p) in enumerate(js, 1):
        dp[i] = max(dp[i - 1], dp[bisect.bisect_right(ends, s)] + p)
    return dp[-1]


def subsets(st, en, pr):
    n, best = len(st), 0
    for mask in range(1 << n):
        chosen = sorted((st[i], en[i]) for i in range(n) if mask >> i & 1)
        if all(a[1] <= b[0] for a, b in zip(chosen, chosen[1:])):
            best = max(best, sum(pr[i] for i in range(n) if mask >> i & 1))
    return best


def quadratic(st, en, pr):
    js = sorted(zip(en, st, pr))
    dp = [0] * (len(js) + 1)
    for i in range(1, len(js) + 1):
        e, s, p = js[i - 1]
        best_prev = max([dp[j + 1] for j in range(i - 1) if js[j][0] <= s], default=0)
        dp[i] = max(dp[i - 1], best_prev + p)
    return dp[-1]


def out(st, en, pr):
    n = len(st)
    assert 1 <= n <= 5 * 10**4 and len(en) == n and len(pr) == n
    assert all(1 <= s < e <= TMAX for s, e in zip(st, en))
    assert all(1 <= p <= 10**4 for p in pr)
    if n <= 12:
        assert ref(st, en, pr) == subsets(st, en, pr)
    elif n <= 400:
        assert ref(st, en, pr) == quadratic(st, en, pr)
    emit(st, en, pr)


out([1], [2], [1])
out([1], [TMAX], [10**4])
out([1, 2], [2, 3], [5, 5])
out([1, 2], [3, 4], [5, 6])
out([1, 1, 1], [TMAX, TMAX, TMAX], [3, 9, 4])
out([1, 3, 5, 7], [3, 5, 7, 9], [1, 1, 1, 1])
out([1, 2, 3, 4], [10, 3, 4, 5], [100, 30, 30, 30])
out([1, 2, 3, 4], [10, 3, 4, 5], [100, 40, 30, 40])
out([4, 2, 4, 8, 2], [5, 5, 5, 10, 8], [1, 2, 8, 10, 4])
out([6, 15, 7, 11, 1, 3, 16, 2], [19, 18, 19, 16, 10, 8, 19, 8], [2, 9, 1, 19, 5, 7, 3, 19])
out(list(range(1, 21)), list(range(2, 22)), [10**4] * 20)
for n in range(2, 13):
    out(*jobs(n, 30, r.choice([2, 5, 15])))
for _ in range(25):
    n = r.randint(13, 400)
    out(*jobs(n, r.choice([n, 10 * n, TMAX]), r.choice([3, 50, 10**4, TMAX]), r.choice([10, 10**4])))
for _ in range(5):
    out(*jobs(r.randint(1000, 4000), TMAX, r.choice([10**5, 10**7, 10**8])))
# Large cases: long overlapping jobs make the backward scan in the O(n^2) DP walk far.
out(*jobs(5 * 10**4, TMAX, TMAX))
out(*jobs(5 * 10**4, 10**5, 10**5))
n = 5 * 10**4
out([i + 1 for i in range(n)], [n + 1 + i for i in range(n)], [r.randint(1, 10**4) for _ in range(n)])
