import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(133)


def to_adj(n, edges, shuffle=True):
    adj = [[] for _ in range(n)]
    for a, b in edges:
        adj[a].append(b + 1)
        adj[b].append(a + 1)
    if shuffle:
        for lst in adj:
            r.shuffle(lst)
    return adj


def connected(n, adj):
    if n == 0:
        return True
    seen, stack = {1}, [1]
    while stack:
        v = stack.pop()
        for w in adj[v - 1]:
            if w not in seen:
                seen.add(w)
                stack.append(w)
    return len(seen) == n


def random_graph(n, extra):
    """A random spanning tree on n nodes plus `extra` additional distinct edges."""
    while True:
        edges = set()
        order = list(range(n))
        r.shuffle(order)
        for i in range(1, n):
            a, b = order[i], order[r.randrange(i)]
            edges.add((min(a, b), max(a, b)))
        max_edges = n * (n - 1) // 2
        target = min(max_edges, len(edges) + extra)
        while len(edges) < target:
            a, b = r.sample(range(n), 2)
            edges.add((min(a, b), max(a, b)))
        adj = to_adj(n, sorted(edges))
        if connected(n, adj) and all(len(set(x)) == len(x) for x in adj):
            return adj


emit([])
emit([[]])
emit([[2], [1]])
emit([[2, 3], [1, 3], [1, 2]])
emit(to_adj(5, [(i, i + 1) for i in range(4)], False))           # path
emit(to_adj(6, [(i, (i + 1) % 6) for i in range(6)], False))     # cycle
emit(to_adj(7, [(0, i) for i in range(1, 7)], False))            # star centered at 1
emit(to_adj(7, [(6, i) for i in range(6)], False))               # star centered at 7
emit(to_adj(6, [(a, b) for a in range(6) for b in range(a + 1, 6)], False))  # complete K6
for n in range(2, 12):
    emit(random_graph(n, r.randint(0, n)))
for _ in range(30):
    n = r.randint(10, 60)
    emit(random_graph(n, r.randint(0, 2 * n)))
emit(to_adj(100, [(i, i + 1) for i in range(99)], False))        # long path
emit(to_adj(100, [(a, b) for a in range(100) for b in range(a + 1, 100)], False))  # K100
for _ in range(5):
    emit(random_graph(100, r.randint(50, 600)))
