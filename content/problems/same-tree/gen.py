import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(100)
V = 10**4


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
        out += [node.val]
        queue += [node.left, node.right]
    while out and out[-1] is None:
        out.pop()
    return out


def nodes(root):
    out, stack = [], [root] if root else []
    while stack:
        node = stack.pop()
        out.append(node)
        stack += [c for c in (node.left, node.right) if c]
    return out


def random_tree(n, lo=-V, hi=V, null_prob=0.3):
    return tree_level_order([r.randint(lo, hi) for _ in range(n)], r, null_prob)


def mutate(arr):
    """Returns a different tree that differs from `arr` in one small, local way."""
    while True:
        root = build(arr)
        all_nodes = nodes(root)
        kind = r.choice(["value", "drop_leaf", "add_leaf", "swap"])
        node = r.choice(all_nodes)
        if kind == "value":
            node.val = r.choice([node.val + 1, node.val - 1, -node.val, r.randint(-V, V)])
            node.val = max(-V, min(V, node.val))
        elif kind == "drop_leaf" and len(all_nodes) > 1:
            for parent in all_nodes:
                if parent.left and not parent.left.left and not parent.left.right and r.random() < 0.5:
                    parent.left = None
                    break
                if parent.right and not parent.right.left and not parent.right.right:
                    parent.right = None
                    break
        elif kind == "add_leaf" and len(all_nodes) < 100:
            if node.left is None:
                node.left = T(r.randint(-V, V))
            elif node.right is None:
                node.right = T(r.randint(-V, V))
        elif kind == "swap":
            node.left, node.right = node.right, node.left
        out = dump(root)
        if out != arr:
            return out


def mirror(arr):
    root = build(arr)
    for node in nodes(root):
        node.left, node.right = node.right, node.left
    return dump(root)


def path(n, side, val=lambda i: i):
    out = [val(0)]
    for i in range(1, n):
        out += [val(i), None] if side == "left" else [None, val(i)]
    return out


# Edge cases: empty trees, single nodes, same values in different shapes.
emit([], [])
emit([], [0])
emit([0], [])
emit([1], [1])
emit([1], [2])
emit([V], [V])
emit([-V], [V])
emit([1, 2], [1, None, 2])
emit([1, None, 2], [1, None, 2])
emit([1, 2, 3], [1, 3, 2])
emit([1, 1], [1, None, 1])
emit([0, 0, 0, 0], [0, 0, 0, None, 0])
emit([5, 5, 5, 5, 5, 5, 5], [5, 5, 5, 5, 5, 5, 5])
emit(path(100, "left"), path(100, "left"))
emit(path(100, "left"), path(100, "right"))
emit(path(100, "right"), path(99, "right"))
emit(path(100, "right"), path(100, "right", lambda i: 7 if i == 99 else i))
full = list(range(1, 101))
emit(full, full)
emit(full, full[:-1])
# Small random pairs: equal, mutated, mirrored.
for n in range(1, 30):
    a = random_tree(n, -10, 10)
    kind = n % 3
    if kind == 0:
        emit(a, list(a))
    elif kind == 1:
        emit(a, mutate(a))
    else:
        emit(a, mirror(a))
for _ in range(30):
    a = random_tree(r.randint(1, 100), r.choice([-V, -3]), r.choice([3, V]), r.choice([0.1, 0.3, 0.45]))
    if r.random() < 0.5:
        emit(a, list(a))
    else:
        b = mutate(a)
        emit(*((a, b) if r.random() < 0.5 else (b, a)))
for _ in range(6):
    a = random_tree(100, null_prob=r.choice([0.0, 0.3]))
    emit(a, list(a))
    emit(a, mutate(a))
