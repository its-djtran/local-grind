import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(323)


def random_edges(n, m, groups=None):
    """m distinct edges [a, b] with a < b. If `groups` is given (a list of node lists), edges are
    only drawn inside a group, so components never merge across groups."""
    pool = groups or [list(range(n))]
    pool = [g for g in pool if len(g) >= 2]
    cap = sum(len(g) * (len(g) - 1) // 2 for g in pool)
    m = min(m, cap)
    edges = set()
    while len(edges) < m:
        g = r.choice(pool)
        a, b = r.sample(g, 2)
        edges.add((min(a, b), max(a, b)))
    out = [list(e) for e in edges]
    r.shuffle(out)
    return out


def partition(n, k):
    nodes = list(range(n))
    r.shuffle(nodes)
    cuts = sorted(r.sample(range(1, n), k - 1)) if k > 1 else []
    return [nodes[i:j] for i, j in zip([0] + cuts, cuts + [n])]


def forest(n, k, extra):
    """Exactly k components: a spanning tree inside each part plus `extra` in-part edges."""
    parts = partition(n, k)
    edges = set()
    for p in parts:
        g = list(p)
        r.shuffle(g)
        for i in range(1, len(g)):
            a, b = g[i], g[r.randrange(i)]
            edges.add((min(a, b), max(a, b)))
    cap = sum(len(p) * (len(p) - 1) // 2 for p in parts)
    target = min(5000, len(edges) + extra, cap)
    while len(edges) < target:
        p = r.choice(parts)
        if len(p) < 2:
            continue
        a, b = r.sample(p, 2)
        edges.add((min(a, b), max(a, b)))
    out = [list(e) for e in edges]
    r.shuffle(out)
    return out


def count_brute(n, edges):
    label = list(range(n))
    for a, b in edges:
        la, lb = label[a], label[b]
        if la != lb:
            label = [la if x == lb else x for x in label]
    return len(set(label))


emit(2, [[0, 1]])
emit(3, [[0, 1]])
emit(3, [[1, 2]])
emit(4, [[0, 1], [2, 3]])
emit(4, [[0, 1], [1, 2], [0, 2]])
emit(6, [[0, 5]])
emit(5, [[0, 1], [0, 2], [0, 3], [0, 4]])
emit(2000, [[0, 1999]])
emit(2000, [[i, i + 1] for i in range(1999)])
emit(2000, [[0, i] for i in range(1, 2000)])
for _ in range(30):
    n = r.randint(2, 12)
    emit(n, random_edges(n, r.randint(1, n + 2)))
for _ in range(15):
    n = r.randint(20, 400)
    k = r.randint(1, n // 2)
    e = forest(n, k, r.randint(0, n))
    if not e:
        e = random_edges(n, 1)
    emit(n, e)

# Large cases: n = 2000 with the maximum number of edges.
for k in (1, 2, 10, 300, 1000):
    emit(2000, forest(2000, k, 5000))
emit(2000, random_edges(2000, 5000))
emit(2000, random_edges(2000, 5000, partition(2000, 1500)))
# A long path listed edge by edge makes a relabel-everything approach do V work per edge.
edges = {(i, i + 1) for i in range(1999)}
while len(edges) < 5000:
    a, b = sorted(r.sample(range(2000), 2))
    edges.add((a, b))
emit(2000, [list(e) for e in sorted(edges, key=lambda e: (e[1] - e[0] != 1, e))])
for _ in range(4):
    emit(2000, [[i, i + 1] for i in range(1999)][::-1])
for _ in range(3):
    emit(2000, forest(2000, r.randint(1, 5), 5000))
