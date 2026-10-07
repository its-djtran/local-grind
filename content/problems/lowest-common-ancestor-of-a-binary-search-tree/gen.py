import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import bst_level_order, emit, rng

r = rng(235)
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


def brute_lca(par, p, q):
    seen = set()
    while p is not None:
        seen.add(p)
        p = par[p]
    while q not in seen:
        q = par[q]
    return q


def bst_walk(level, p, q):
    """The ordering-based walk, run on the level-order array, for cross-checking."""
    kids = children(level)
    node = level[0]
    while True:
        if p < node and q < node:
            node = kids[node][0]
        elif p > node and q > node:
            node = kids[node][1]
        else:
            return node


def children(level):
    kids = {level[0]: [None, None]}
    q = deque([level[0]])
    i = 1
    while q and i < len(level):
        v = q.popleft()
        for side in range(2):
            if i < len(level) and level[i] is not None:
                kids[v][side] = level[i]
                kids[level[i]] = [None, None]
                q.append(level[i])
            i += 1
    return kids


def random_bst(values):
    root = None
    for v in values:
        node = [v, None, None]
        if root is None:
            root = node
            continue
        cur = root
        while True:
            side = 1 if v < cur[0] else 2
            if cur[side] is None:
                cur[side] = node
                break
            cur = cur[side]
    return root


def chain(values, side):
    root = cur = [values[0], None, None]
    for v in values[1:]:
        cur[side] = [v, None, None]
        cur = cur[side]
    return root


def is_bst(level):
    kids = children(level)
    out, stack, cur = [], [], level[0]
    while stack or cur is not None:
        while cur is not None:
            stack.append(cur)
            cur = kids[cur][0]
        cur = stack.pop()
        out.append(cur)
        cur = kids[cur][1]
    count = sum(v is not None for v in level)
    return len(out) == count and all(a < b for a, b in zip(out, out[1:]))


def case(level, p, q):
    assert p != q
    assert is_bst(level)
    par = parents(level)
    assert p in par and q in par
    assert brute_lca(par, p, q) == bst_walk(level, p, q)
    emit(level, p, q)


def pick(level):
    vals = [v for v in level if v is not None]
    p, q = r.sample(vals, 2)
    return p, q


def ancestor_pair(level):
    """Picks p as a proper ancestor of q."""
    par = parents(level)
    vals = [v for v in par if par[v] is not None]
    q = r.choice(vals)
    anc = []
    cur = par[q]
    while cur is not None:
        anc.append(cur)
        cur = par[cur]
    return r.choice(anc), q


# Edge cases.
case([2, 1], 2, 1)
case([2, 1], 1, 2)
case([1, None, 2], 1, 2)
case([1, None, 2], 2, 1)
case([0, -VMAX, VMAX], -VMAX, VMAX)
case([0, -VMAX, VMAX], VMAX, 0)
case([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5], 3, 5)
case([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5], 0, 5)
case([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5], 5, 7)
case([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5], 9, 7)
case([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5], 3, 6)
case([3, 1, 4, None, 2], 2, 4)

# Small random trees.
for _ in range(25):
    n = r.randint(2, 20)
    level = serialize(random_bst(r.sample(range(-50, 51), n)))
    case(level, *pick(level))
for _ in range(10):
    n = r.randint(3, 20)
    level = serialize(random_bst(r.sample(range(-50, 51), n)))
    p, q = ancestor_pair(level)
    case(level, *(p, q) if r.random() < 0.5 else (q, p))

# Medium trees over the full value range.
for _ in range(10):
    n = r.randint(100, 3000)
    level = serialize(random_bst(r.sample(range(-VMAX, VMAX + 1), n)))
    case(level, *pick(level))

# Large cases: long paths with both targets deep down defeat subtree-searching solutions.
level = serialize(chain(list(range(50000)), 2))
case(level, 49998, 49999)
level = serialize(chain(list(range(30000, 0, -1)), 1))
case(level, 2, 1)
level = bst_level_order(list(range(10**5)))
case(level, *pick(level))
level = bst_level_order(list(range(-VMAX, VMAX, 2 * 10**6)))
case(level, -VMAX, VMAX - 2 * 10**6)
level = serialize(random_bst(r.sample(range(-VMAX, VMAX + 1), 30000)))
case(level, *pick(level))
