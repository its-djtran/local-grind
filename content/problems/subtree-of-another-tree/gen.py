import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(572)
V = 10**4
M, S = 2000, 1000  # node limits for root and subRoot


class T:
    __slots__ = ("val", "left", "right")

    def __init__(self, val, left=None, right=None):
        self.val, self.left, self.right = val, left, right


def build(arr):
    if not arr:
        return None
    root, i = T(arr[0]), 1
    queue = deque([root])
    while queue and i < len(arr):
        node = queue.popleft()
        if i < len(arr) and arr[i] is not None:
            node.left = T(arr[i])
            queue.append(node.left)
        i += 1
        if i < len(arr) and arr[i] is not None:
            node.right = T(arr[i])
            queue.append(node.right)
        i += 1
    return root


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


def subtree_sizes(root):
    """Pairs (node, size of its subtree) for every node, computed without recursion."""
    order, stack = [], [root]
    while stack:
        node = stack.pop()
        order.append(node)
        stack += [c for c in (node.left, node.right) if c]
    size = {}
    for node in reversed(order):
        size[id(node)] = 1 + sum(size[id(c)] for c in (node.left, node.right) if c)
    return [(node, size[id(node)]) for node in order]


def count(arr):
    return sum(v is not None for v in arr)


def random_tree(n, lo=-V, hi=V, null_prob=0.3):
    return tree_level_order([r.randint(lo, hi) for _ in range(n)], r, null_prob)


def pick_subtree(arr, min_size=1):
    """Level-order array of a random subtree of `arr` with at most S nodes (prefers >= min_size)."""
    cands = [(n, s) for n, s in subtree_sizes(build(arr)) if s <= S]
    big = [c for c in cands if c[1] >= min_size]
    node, _ = r.choice(big or cands)
    return dump(node)


def mutate(arr):
    """A small local change to a tree: new value at one node, or one leaf added or removed."""
    while True:
        root = build(arr)
        all_nodes = [n for n, _ in subtree_sizes(root)]
        node = r.choice(all_nodes)
        kind = r.choice(["value", "add", "drop"])
        if kind == "value":
            node.val = max(-V, min(V, node.val + r.choice([-1, 1, 7])))
        elif kind == "add" and len(all_nodes) < S:
            if node.left is None:
                node.left = T(r.randint(-V, V))
            elif node.right is None:
                node.right = T(r.randint(-V, V))
        elif kind == "drop" and len(all_nodes) > 1:
            for parent in all_nodes:
                for side in ("left", "right"):
                    c = getattr(parent, side)
                    if c and not c.left and not c.right and r.random() < 0.3:
                        setattr(parent, side, None)
                        break
        out = dump(root)
        if out != arr:
            return out


def case(n, lo, hi, null_prob, want_match):
    root = random_tree(n, lo, hi, null_prob)
    sub = pick_subtree(root, r.randint(1, max(1, count(root) // 3)))
    if not want_match:
        sub = mutate(sub)
    assert 1 <= count(root) <= M and 1 <= count(sub) <= S
    return root, sub


def path(vals, side):
    out = [vals[0]]
    for v in vals[1:]:
        out += [v, None] if side == "left" else [None, v]
    return out


# Edge cases.
emit([1], [1])
emit([1], [2])
emit([12], [2])
emit([2], [12])
emit([1, None, 12], [2])
emit([12, None, 3], [2, None, 3])
emit([1, 1], [1])
emit([1, 2], [1])
emit([1, 2, 3], [1, 2, 3])
emit([1, 2, 3], [1, 2])
emit([1, None, 2], [2])
emit([1, 2], [1, None, 2])
emit([4, 4, None, 4], [4, 4])
emit([3, 4, 5, 1, None, 2], [3, 1, 2])
emit([-V, V, -V], [V])
emit([0, 0, 0, 0, 0, 0, 0], [0, 0, 0])
emit([0, 0, 0, 0, None, 0, 0], [0, 0, 0])
emit([1, 2, 3, 4, 5, 6, 7], [3, 6, 7])
emit([1, 2, 3, 4, 5, 6, 7], [3, 7, 6])
# Small random cases, half of them true matches.
for n in range(2, 40):
    lo, hi = (-3, 3) if n % 4 == 0 else (-V, V)
    emit(*case(n, lo, hi, 0.3, n % 2 == 0))
for i in range(24):
    lo, hi = r.choice([(-V, V), (0, 2), (-50, 50)])
    emit(*case(r.randint(40, M), lo, hi, r.choice([0.1, 0.3, 0.45]), i % 2 == 0))
# Large cases: long runs of equal values force a node-by-node comparison at every start.
zeros = path([0] * M, "left")
emit(zeros, path([0] * S, "left"))
emit(zeros, path([0] * (S - 1) + [1], "left"))
emit(path([0] * (M - 1) + [1], "right"), path([0] * (S - 1) + [1], "right"))
emit(path([0] * M, "right"), path([0] * (S - 1) + [5], "right"))
emit(path([7] * M, "left"), path([7] * (S - 1), "left") + [None, 7])
emit(tree_level_order([0] * M), tree_level_order([0] * (S - 1) + [1]))
emit(tree_level_order([0] * M), tree_level_order([0] * 511))
