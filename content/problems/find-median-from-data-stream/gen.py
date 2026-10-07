import heapq
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(295)
LIM = 10**5
MAX_CALLS = 5 * 10**4


def run_ref(ops, args):
    lo, hi, res = [], [], [None]
    for op, a in zip(ops[1:], args[1:]):
        if op == "addNum":
            heapq.heappush(hi, -heapq.heappushpop(lo, -a[0]))
            if len(hi) > len(lo):
                heapq.heappush(lo, -heapq.heappop(hi))
            res.append(None)
        else:
            res.append(float(-lo[0]) if len(lo) > len(hi) else (-lo[0] + hi[0]) / 2)
    return res


def run_brute(ops, args):
    nums, res = [], [None]
    for op, a in zip(ops[1:], args[1:]):
        if op == "addNum":
            nums.append(a[0])
            res.append(None)
        else:
            s, n = sorted(nums), len(nums)
            res.append(float(s[n // 2]) if n % 2 else (s[n // 2 - 1] + s[n // 2]) / 2)
    return res


def out(values, query_after):
    """values: numbers to add in order; query_after[i] = number of findMedian calls after values[i]."""
    ops, args = ["MedianFinder"], [[]]
    for v, q in zip(values, query_after):
        ops.append("addNum")
        args.append([v])
        ops += ["findMedian"] * q
        args += [[]] * q
    assert 1 <= len(ops) - 1 <= MAX_CALLS
    assert ops[1] == "addNum" and "findMedian" in ops
    assert all(-LIM <= v <= LIM for v in values)
    if len(ops) <= 3000:
        assert run_ref(ops, args) == run_brute(ops, args)
    emit(ops, args)


def every(values):
    return out(values, [1] * len(values))


def rand_queries(n, p):
    q = [1 if r.random() < p else 0 for _ in range(n)]
    q[-1] = 1
    return q


every([5])
every([-LIM])
every([LIM, -LIM])
every([1, 2, 3, 4, 5])
every([5, 4, 3, 2, 1])
every([0, 0, 0, 0])
every([-1, -2])
every([-3, 2])
every([1, 1, 2, 2, 3, 3])
every([LIM, LIM - 1, LIM, LIM - 1])
out([6, 10, 2, 6, 5, 0, 6, 3, 1, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2])
out([2, 3, 4], [0, 1, 1])
for n in range(1, 16):
    every([r.randint(-10, 10) for _ in range(n)])
for _ in range(25):
    n = r.randint(16, 1000)
    lim = r.choice([3, 100, LIM])
    out([r.randint(-lim, lim) for _ in range(n)], rand_queries(n, r.choice([0.1, 0.5, 1.0])))
for _ in range(4):
    out([r.randint(0, 100) for _ in range(500)], rand_queries(500, 0.5))
for _ in range(4):
    n = r.randint(2000, 8000)
    out([r.randint(-LIM, LIM) for _ in range(n)], rand_queries(n, 0.5))
# Large cases: the full call budget split between additions and queries.
H = MAX_CALLS // 2
out([r.randint(-LIM, LIM) for _ in range(H)], [1] * H)
out(list(range(-1, -H - 1, -1)), [1] * H)
out([r.choice([r.randint(0, 100), r.randint(-LIM, LIM)]) for _ in range(H)], [1] * H)
