import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(207)


def has_cycle_brute(n, pre):
    graph = [[] for _ in range(n)]
    for a, b in pre:
        graph[b].append(a)
    for s in range(n):
        seen, stack = set(), list(graph[s])
        while stack:
            v = stack.pop()
            if v == s:
                return True
            if v not in seen:
                seen.add(v)
                stack.extend(graph[v])
    return False


def dag(n, m):
    """m distinct prerequisite pairs consistent with a random hidden order (acyclic)."""
    order = list(range(n))
    r.shuffle(order)
    m = min(m, n * (n - 1) // 2)
    pairs = set()
    while len(pairs) < m:
        i, j = sorted(r.sample(range(n), 2))
        pairs.add((order[j], order[i]))  # order[i] comes first, so it is the prerequisite
    out = [list(p) for p in pairs]
    r.shuffle(out)
    return out


def with_cycle(n, m, cycle_len):
    """A DAG with one back edge closing a cycle of the requested length."""
    order = list(range(n))
    r.shuffle(order)
    pairs = set()
    start = r.randrange(n - cycle_len + 1)
    for k in range(start, start + cycle_len - 1):
        pairs.add((order[k + 1], order[k]))
    pairs.add((order[start], order[start + cycle_len - 1]))
    m = min(max(m, len(pairs)), n * (n - 1) // 2)
    while len(pairs) < m:
        i, j = sorted(r.sample(range(n), 2))
        if (order[i], order[j]) not in pairs:
            pairs.add((order[j], order[i]))
    out = [list(p) for p in pairs]
    r.shuffle(out)
    return out


def random_pairs(n, m):
    m = min(m, n * (n - 1))
    pairs = set()
    while len(pairs) < m:
        a, b = r.sample(range(n), 2)
        pairs.add((a, b))
    return [list(p) for p in pairs]


emit(1, [])
emit(2, [])
emit(3, [[1, 0], [2, 1]])
emit(3, [[1, 0], [2, 1], [0, 2]])
emit(2, [[0, 1]])
emit(4, [[1, 0], [2, 0], [3, 1], [3, 2]])
emit(4, [[1, 0], [2, 1], [3, 2], [1, 3]])
emit(5, [[1, 4], [2, 4], [3, 1], [3, 2]])
emit(2000, [])
emit(2000, [[i + 1, i] for i in range(1999)])
emit(2000, [[i + 1, i] for i in range(1999)] + [[0, 1999]])

for _ in range(30):
    n = r.randint(2, 12)
    kind = r.random()
    if kind < 0.4:
        pre = dag(n, r.randint(0, n * (n - 1) // 2))
    elif kind < 0.7:
        pre = with_cycle(n, r.randint(0, n * (n - 1) // 2), r.randint(2, n))
    else:
        pre = random_pairs(n, r.randint(0, n + 3))
    emit(n, pre)
for _ in range(15):
    n = r.randint(20, 300)
    if r.random() < 0.5:
        emit(n, dag(n, r.randint(0, min(5000, 3 * n))))
    else:
        emit(n, with_cycle(n, r.randint(n, min(5000, 3 * n)), r.randint(2, min(n, 50))))

# Large cases. A layered DAG with many paths makes an un-memoized DFS revisit nodes many times,
# and a per-course traversal costs V * (V + E).
n = 2000
emit(n, dag(n, 5000))
emit(n, dag(n, 5000))
layers = [list(range(k * 20, k * 20 + 20)) for k in range(100)]
pre = []
for k in range(99):
    for a in layers[k + 1]:
        for b in r.sample(layers[k], 2):
            pre.append([a, b])
emit(n, pre[:5000])
emit(n, with_cycle(n, 5000, 2000))
emit(n, with_cycle(n, 5000, 2))
chain = [[i + 1, i] for i in range(1999)]
extra = set()
while len(extra) < 3001:
    a, b = r.randrange(1000, 2000), r.randrange(0, 1000)
    if a != b + 1:
        extra.add((a, b))
emit(n, chain + [list(p) for p in extra])


def total_order(n, m, offset=0):
    """A chain 0 -> 1 -> ... -> n-1 (shifted by offset) plus random forward edges: acyclic, and
    every course has many courses depending on it."""
    pairs = {(i + 1, i) for i in range(n - 1)}
    while len(pairs) < m:
        i, j = sorted(r.sample(range(n), 2))
        pairs.add((j, i))
    out = [[a + offset, b + offset] for a, b in pairs]
    r.shuffle(out)
    return out


emit(2000, total_order(2000, 5000))
emit(2000, total_order(2000, 5000))
# A long acyclic prefix whose courses all come first, with a cycle hidden among the last ones.
pre = total_order(1900, 4880)
pre += [[1900 + (i + 1) % 100, 1900 + i] for i in range(100)]
r.shuffle(pre)
emit(2000, pre)

for _ in range(3):
    emit(2000, total_order(2000, 5000))
