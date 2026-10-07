import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(261)


def flip(edges):
    return [[b, a] if r.random() < 0.5 else [a, b] for a, b in edges]


def random_tree(n):
    labels = list(range(n))
    r.shuffle(labels)
    edges = [(labels[i], labels[r.randrange(i)]) for i in range(1, n)]
    r.shuffle(edges)
    return flip(edges)


def path(n, shuffled=False):
    labels = list(range(n))
    r.shuffle(labels)
    edges = [(labels[i], labels[i + 1]) for i in range(n - 1)]
    if shuffled:
        r.shuffle(edges)
    return flip(edges)


def add_edges(n, edges, k):
    have = {frozenset(e) for e in edges}
    out = [list(e) for e in edges]
    k = min(k, n * (n - 1) // 2 - len(have))
    while k > 0:
        a, b = r.sample(range(n), 2)
        if frozenset((a, b)) not in have:
            have.add(frozenset((a, b)))
            out.append([a, b])
            k -= 1
    return out


def drop_and_add(n, edges):
    """Same edge count as a tree but disconnected (and therefore with a cycle)."""
    while True:
        e = [list(x) for x in edges]
        e.pop(r.randrange(len(e)))
        e = add_edges(n, e, 1)
        if not tree_brute(n, e):
            return e


def tree_brute(n, edges):
    graph = [[] for _ in range(n)]
    for a, b in edges:
        graph[a].append(b)
        graph[b].append(a)
    seen, stack = {0}, [0]
    while stack:
        for w in graph[stack.pop()]:
            if w not in seen:
                seen.add(w)
                stack.append(w)
    return len(seen) == n and len(edges) == n - 1


emit(1, [])
emit(2, [])
emit(2, [[0, 1]])
emit(2, [[1, 0]])
emit(3, [[0, 1]])
emit(3, [[0, 1], [1, 2], [2, 0]])
emit(4, [[0, 1], [2, 3]])
emit(4, [[0, 1], [1, 2], [2, 0]])  # n - 1 edges but node 3 is isolated
emit(4, [[1, 2], [2, 3], [3, 1]])
emit(4, [[0, 1], [0, 2], [0, 3]])
emit(5, [[0, 1], [1, 2], [2, 3], [3, 4]])
emit(2000, [])
for _ in range(30):
    n = r.randint(1, 12)
    kind = r.randrange(4)
    if kind == 0 or n == 1:
        e = random_tree(n)
    elif kind == 1:
        e = add_edges(n, random_tree(n), r.randint(1, 3))
    elif kind == 2:
        e = drop_and_add(n, random_tree(n)) if n >= 4 else []
    else:
        e = random_tree(n)
        e.pop(r.randrange(len(e)))
    emit(n, e)
for _ in range(12):
    n = r.randint(20, 500)
    kind = r.randrange(3)
    e = random_tree(n) if kind == 0 else (add_edges(n, random_tree(n), 1) if kind == 1 else drop_and_add(n, random_tree(n)))
    emit(n, e)

# Large: long paths and trees at the limit; a repeated-search solution pays O(V) per edge.
emit(2000, path(2000))
emit(2000, path(2000))
emit(2000, path(2000, True))
emit(2000, random_tree(2000))
emit(2000, random_tree(2000))
e = path(2000)
emit(2000, e + [[e[0][0], e[-1][1]]])  # the last edge closes one big cycle
emit(2000, drop_and_add(2000, path(2000)))
emit(2000, add_edges(2000, path(2000), 3001))
emit(2000, add_edges(2000, random_tree(2000), 1))
for _ in range(6):
    emit(2000, path(2000))
