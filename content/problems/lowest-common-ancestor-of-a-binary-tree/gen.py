import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(236)
VMAX = 10**9

# Trees are built as [val, left, right] lists and serialized to LeetCode level order.


def serialize(root):
    out, q = [], deque([root])
    while q:
        node = q.popleft()
        if node is None:
            out.append(None)
            continue
        out.append(node[0])
        q.append(node[1])
        q.append(node[2])
    while out and out[-1] is None:
        out.pop()
    return out


def parents(level):
    """Maps each value of a level-order tree to its parent's value (root -> None)."""
    par = {level[0]: None}
    q = deque([level[0]])
    i = 1
    while q and i < len(level):
        v = q.popleft()
        for _ in range(2):
            if i < len(level) and level[i] is not None:
                par[level[i]] = v
                q.append(level[i])
            i += 1
    return par


def ancestors(par, v):
    out = []
    while v is not None:
        out.append(v)
        v = par[v]
    return out


def brute_lca(par, p, q):
    seen = set(ancestors(par, p))
    while q not in seen:
        q = par[q]
    return q


def random_path(values):
    """One long path whose turns are random: every node has a single child."""
    root = cur = [values[0], None, None]
    for v in values[1:]:
        side = r.choice((1, 2))
        cur[side] = [v, None, None]
        cur = cur[side]
    return root


def case(level, p, q):
    par = parents(level)
    assert len(par) == sum(v is not None for v in level)  # values unique
    assert p != q and p in par and q in par
    brute_lca(par, p, q)
    emit(level, p, q)


def pick(level):
    vals = [v for v in level if v is not None]
    return tuple(r.sample(vals, 2))


def ancestor_pair(level):
    par = parents(level)
    q = r.choice([v for v in par if par[v] is not None])
    p = r.choice(ancestors(par, par[q]))
    return (p, q) if r.random() < 0.5 else (q, p)


def deep_pair(level):
    """Two nodes from the bottom of the tree, so the answer is far from the root."""
    par = parents(level)
    depth = {}
    for v in par:
        depth[v] = len(ancestors(par, v)) if len(par) < 3000 else 0
    if len(par) >= 3000:
        order = [v for v in level if v is not None]  # BFS order: deepest last
        return order[-1], order[-2]
    vals = sorted(par, key=lambda v: -depth[v])
    return vals[0], vals[1]


def random_tree(n, lo, hi, null_prob, min_nodes=2):
    """Random shape with distinct values; retried until it keeps at least min_nodes nodes."""
    while True:
        level = tree_level_order(r.sample(range(lo, hi + 1), n), r, null_prob)
        if sum(v is not None for v in level) >= min_nodes:
            return level


# Edge cases.
case([1, 2], 1, 2)
case([1, 2], 2, 1)
case([1, None, 2], 2, 1)
case([-VMAX, VMAX], VMAX, -VMAX)
case([0, -VMAX, VMAX], -VMAX, VMAX)
case([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], 6, 4)
case([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], 7, 8)
case([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], 0, 8)
case([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], 3, 4)
case([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], 7, 2)
case([1, 2, 3, None, 4], 4, 3)
case([37, -34, -48, None, -100, -101, 48, None, None, None, None, -54, None, -71, -22, None, None, None, 8], -71, 8)

# Small random trees.
for _ in range(25):
    level = random_tree(r.randint(2, 20), -50, 50, r.choice([0.0, 0.2, 0.4]))
    case(level, *pick(level))
for _ in range(10):
    level = random_tree(r.randint(3, 20), -50, 50, 0.3)
    case(level, *ancestor_pair(level))
for _ in range(5):
    level = serialize(random_path(r.sample(range(-50, 51), r.randint(2, 15))))
    case(level, *pick(level))

# Medium trees over the full value range.
for _ in range(8):
    level = random_tree(r.randint(100, 3000), -VMAX, VMAX, r.choice([0.1, 0.3, 0.45]))
    case(level, *(pick(level) if r.random() < 0.5 else deep_pair(level)))

# Large cases: long paths with both targets near the bottom defeat subtree-searching solutions.
values = r.sample(range(10**5), 10**5)
level = serialize(random_path(values[:50000]))
case(level, *deep_pair(level))
level = serialize(random_path(values[:30000]))
case(level, *pick(level))
level = tree_level_order(values)  # complete tree at the size limit
case(level, *deep_pair(level))
level = random_tree(10**5, 0, 10**5 - 1, 0.45, 20000)
case(level, *deep_pair(level))
level = random_tree(20000, -VMAX, VMAX, 0.3, 10000)
case(level, *pick(level))
