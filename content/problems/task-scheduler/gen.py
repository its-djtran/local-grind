import string
import sys
from collections import Counter
from functools import lru_cache

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(621)
LETTERS = string.ascii_uppercase


def formula(tasks, n):
    counts = Counter(tasks).values()
    f = max(counts)
    return max(len(tasks), (f - 1) * (n + 1) + sum(1 for c in counts if c == f))


def exact(tasks, n):
    """Exhaustive search over (remaining count, cooldown) states; tiny inputs only."""

    @lru_cache(maxsize=None)
    def go(state):
        if not state:
            return 0
        best = None
        options = [i for i, (_, cd) in enumerate(state) if cd == 0]
        moves = []
        for i in options:
            nxt = []
            for j, (c, cd) in enumerate(state):
                if j == i:
                    if c > 1:
                        nxt.append((c - 1, n))
                else:
                    nxt.append((c, max(0, cd - 1)))
            moves.append(tuple(sorted(nxt)))
        if not options:
            moves.append(tuple(sorted((c, cd - 1) for c, cd in state)))
        for m in set(moves):
            v = 1 + go(m)
            best = v if best is None else min(best, v)
        return best

    return go(tuple(sorted((c, 0) for c in Counter(tasks).values())))


def rand_tasks(m, distinct, skew=False):
    pool = r.sample(LETTERS, distinct)
    if skew:
        weights = [2 ** -i for i in range(distinct)]
        return r.choices(pool, weights, k=m)
    return [r.choice(pool) for _ in range(m)]


def out(tasks, n):
    assert 1 <= len(tasks) <= 10**4 and 0 <= n <= 100
    assert all(t in LETTERS for t in tasks)
    if len(tasks) <= 8:
        assert formula(tasks, n) == exact(tasks, n), (tasks, n)
    emit(tasks, n)


out(["A"], 0)
out(["A"], 100)
out(["A", "A"], 0)
out(["A", "A"], 1)
out(["A", "A"], 100)
out(["A", "B"], 100)
out(["A", "A", "A", "B", "B", "B"], 0)
out(["A", "A", "A", "B", "B", "B", "C", "C", "C", "D", "D", "E"], 2)
out(["A", "A", "A", "A", "A", "A", "B", "C", "D", "E", "F", "G"], 2)
out(list(LETTERS), 25)
out(list(LETTERS) * 2, 25)
out(list(LETTERS) * 2, 26)
out(list(LETTERS) * 3, 1)
out(["A", "B", "C", "A", "B"], 2)
for _ in range(20):
    m = r.randint(1, 8)
    out(rand_tasks(m, r.randint(1, min(m, 4))), r.randint(0, 4))
for _ in range(25):
    m = r.randint(9, 500)
    out(rand_tasks(m, r.randint(1, 26), r.random() < 0.5), r.choice([0, 1, 2, r.randint(0, 100)]))
for _ in range(4):
    out(rand_tasks(r.randint(1000, 10**4), r.randint(1, 26), r.random() < 0.5), r.randint(0, 100))
# Large cases where the answer is near 10^6 slots, so slot-by-slot simulation is slow.
out(["A"] * 10**4, 100)
out(["Z"] * 10**4, 99)
out(["A"] * 5000 + ["B"] * 5000, 100)
for f in (9000, 9500, 8000, 9900, 9000, 9700, 8500, 9300, 9950, 9100):
    rest = [r.choice(LETTERS[1:]) for _ in range(10**4 - f - 25)] + list(LETTERS[1:])
    tasks = ["A"] * f + rest
    r.shuffle(tasks)
    out(tasks, r.choice([99, 100]))
for _ in range(4):
    out(rand_tasks(10**4, 26, True), 100)
out(rand_tasks(10**4, 26), 100)
out(rand_tasks(10**4, 26), 0)
