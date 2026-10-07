import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(310)
N = 2 * 10**4


def finish(n, edges):
    """Relabel nodes randomly, randomize edge direction and order."""
    labels = list(range(n))
    r.shuffle(labels)
    out = [[labels[a], labels[b]] if r.random() < 0.5 else [labels[b], labels[a]] for a, b in edges]
    r.shuffle(out)
    return n, out


def random_tree(n):
    return finish(n, [(i, r.randrange(i)) for i in range(1, n)])


def deep_tree(n, window):
    """Each node attaches to one of the last few nodes, producing long paths."""
    return finish(n, [(i, r.randrange(max(0, i - window), i)) for i in range(1, n)])


def path(n):
    return finish(n, [(i, i - 1) for i in range(1, n)])


def star(n):
    return finish(n, [(i, 0) for i in range(1, n)])


def caterpillar(n, spine):
    edges = [(i, i - 1) for i in range(1, spine)]
    edges += [(i, r.randrange(spine)) for i in range(spine, n)]
    return finish(n, edges)


def brute(n, edges):
    graph = [[] for _ in range(n)]
    for a, b in edges:
        graph[a].append(b)
        graph[b].append(a)
    heights = []
    for root in range(n):
        dist = {root: 0}
        stack = [root]
        while stack:
            v = stack.pop()
            for w in graph[v]:
                if w not in dist:
                    dist[w] = dist[v] + 1
                    stack.append(w)
        heights.append(max(dist.values()))
    best = min(heights)
    return sorted(v for v in range(n) if heights[v] == best)


emit(1, [])
emit(2, [[0, 1]])
emit(2, [[1, 0]])
emit(3, [[0, 1], [1, 2]])
emit(3, [[0, 2], [1, 2]])
emit(4, [[0, 1], [1, 2], [2, 3]])
emit(5, [[0, 1], [0, 2], [0, 3], [0, 4]])
emit(7, [[0, 1], [1, 2], [1, 3], [2, 4], [3, 5], [4, 6]])
for _ in range(30):
    emit(*random_tree(r.randint(3, 15)))
for _ in range(8):
    emit(*deep_tree(r.randint(15, 300), 3))
for _ in range(7):
    emit(*random_tree(r.randint(100, 2000)))

# Large trees at the limit: a BFS from every root is quadratic.
emit(*path(N))
emit(*path(N - 1))
emit(*star(N))
emit(*random_tree(N))
emit(*deep_tree(N, 2))
emit(*deep_tree(N, 10))
emit(*caterpillar(N, 5000))
