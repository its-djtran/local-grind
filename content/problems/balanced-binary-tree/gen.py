import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(110)
V = 10**4
N = 5000


class T:
    __slots__ = ("val", "left", "right")

    def __init__(self, val, left=None, right=None):
        self.val, self.left, self.right = val, left, right


def val():
    return r.randint(-V, V)


def dump(root):
    out, queue = [], deque([root])
    while queue:
        node = queue.popleft()
        if node is None:
            out.append(None)
            continue
        out.append(node.val)
        queue += [node.left, node.right]
    while out and out[-1] is None:
        out.pop()
    return out


def postorder(root):
    order, stack = [], [root] if root else []
    while stack:
        node = stack.pop()
        order.append(node)
        stack += [c for c in (node.left, node.right) if c]
    return order[::-1]


def heights(root):
    h = {None: 0}
    for node in postorder(root):
        h[node] = 1 + max(h[node.left], h[node.right])
    return h


def balanced(root):
    h = heights(root)
    return all(abs(h[n.left] - h[n.right]) <= 1 for n in postorder(root))


def size(root):
    return len(postorder(root))


def bal(h, p_full):
    """A random height-balanced tree of height exactly h."""
    if h <= 0:
        return None
    if h == 1:
        return T(val())
    a, b = h - 1, h - 1
    if r.random() > p_full:
        a, b = (h - 1, h - 2) if r.random() < 0.5 else (h - 2, h - 1)
    return T(val(), bal(a, p_full), bal(b, p_full))


def fib_tree(h, flip=False):
    """Minimal (Fibonacci) height-balanced tree: leaves at very different depths."""
    if h <= 0:
        return None
    if h == 1:
        return T(val())
    big, small = fib_tree(h - 1, flip), fib_tree(h - 2, flip)
    return T(val(), small, big) if flip else T(val(), big, small)


def random_balanced(lo, hi):
    while True:
        root = bal(r.randint(2, 13), r.choice([0.2, 0.5, 0.8]))
        if lo <= size(root) <= hi:
            assert balanced(root)
            return root


def unbalance(root):
    """Extends a deepest leaf under a node whose taller side is already one level higher.

    Perfect trees have no such node; there a deepest leaf grows a two-node chain instead.
    Adds at most two nodes.
    """
    h = heights(root)
    cands = [n for n in postorder(root) if abs(h[n.left] - h[n.right]) == 1]
    if cands:
        node = r.choice(cands)
        cur = node.left if h[node.left] > h[node.right] else node.right
    else:
        cur = root
    while cur.left or cur.right:
        cur = cur.left if h[cur.left] >= h[cur.right] else cur.right
    cur.left = T(val())
    if not cands:
        cur.left.left = T(val())
    assert not balanced(root)
    return root


def path(n, sides):
    out = [val()]
    for s in sides[: n - 1]:
        out += [val(), None] if s == "L" else [None, val()]
    return out


def comb(length):
    """A left path whose nodes each carry a right chain matching the remaining path height."""
    nodes = [T(val()) for _ in range(length)]
    for i in range(length - 1):
        nodes[i].left = nodes[i + 1]
        cur = nodes[i].right = T(val())
        for _ in range(length - i - 2):
            cur.right = T(val())
            cur = cur.right
    return nodes[0]


# Edge cases.
emit([])
emit([1])
emit([1, 2])
emit([1, None, 2])
emit([1, 2, None, 3])
emit([1, None, 2, None, 3])
emit([1, 2, 3, 4])
emit([1, 2, 3, 4, None, None, 5])
emit([1, 2, 2, 3, None, None, 3, 4, None, None, 4])
emit([1, 2, 3, 4, 5, None, 6, 7])
emit([1, 2, 3, 4, 5, None, None, 6])
emit([V, -V, V, -V, V, -V, V])
emit(dump(fib_tree(5)))
emit(dump(fib_tree(6, True)))
emit(dump(unbalance(fib_tree(6))))
# Small random cases: balanced, broken, and arbitrary.
for i in range(45):
    kind = i % 3
    if kind == 0:
        emit(dump(random_balanced(2, 40)))
    elif kind == 1:
        emit(dump(unbalance(random_balanced(3, 40))))
    else:
        emit(tree_level_order([val() for _ in range(r.randint(2, 30))], r, 0.25))
for i in range(12):
    root = random_balanced(40, N - 2)
    emit(dump(unbalance(root) if i % 2 else root))
# Large cases near the limit.
emit(path(N, "L" * N))
emit(path(N, "R" * N))
emit(path(N, "LR" * N))
emit(tree_level_order([val() for _ in range(N)]))
emit(dump(fib_tree(17)))
emit(dump(unbalance(fib_tree(17, True))))
emit(dump(comb(99)))
for _ in range(3):
    emit(dump(random_balanced(3000, N)))
    emit(dump(unbalance(random_balanced(3000, N - 2))))
