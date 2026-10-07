import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(199)

# Trees are built as [val, left, right] lists and serialized to LeetCode level order.


def serialize(root):
    if root is None:
        return []
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


def random_shape(n):
    """Attaches each new node to a random free child slot."""
    root = [r.randint(-100, 100), None, None]
    nodes = [root]
    while len(nodes) < n:
        parent = r.choice(nodes)
        side = r.choice((1, 2))
        if parent[side] is None:
            parent[side] = [r.randint(-100, 100), None, None]
            nodes.append(parent[side])
    return root


def path(n, turn):
    """A single path; `turn(i)` picks the side (1 = left, 2 = right) of step i."""
    root = cur = [r.randint(-100, 100), None, None]
    for i in range(1, n):
        side = turn(i)
        cur[side] = [r.randint(-100, 100), None, None]
        cur = cur[side]
    return root


def left_heavy(n_left, n_right):
    """A short right branch next to a long left branch."""
    root = [0, path(n_left, lambda i: 1), path(n_right, lambda i: 2) if n_right else None]
    return root


emit([])
emit([7])
emit([-100])
emit([1, 2])
emit([1, None, 2])
emit([1, 2, 3])
emit([1, 2, 3, 4])
emit([1, 2, 3, None, 5])
emit([1, 2, 3, 4, 5, 6, 7])
emit([0, 0, 0, 0, None, None, 0])
emit([1, 2, None, 3, None, 4])
emit([1, 2, 3, None, 4, None, None, 5, 6])
emit(serialize(left_heavy(10, 3)))
emit(serialize(left_heavy(6, 0)))
emit(serialize(path(15, lambda i: 1 + i % 2)))
emit(serialize(path(15, lambda i: r.choice((1, 2)))))
for n in range(2, 30):
    emit(serialize(random_shape(n)))
for _ in range(20):
    emit(serialize(random_shape(r.randint(30, 100))))
for _ in range(6):
    emit(tree_level_order([r.randint(-100, 100) for _ in range(100)], r, r.choice([0.2, 0.4])))
emit(tree_level_order([r.randint(-100, 100) for _ in range(100)]))
emit(serialize(path(100, lambda i: 1)))
emit(serialize(path(100, lambda i: 2)))
emit(serialize(left_heavy(90, 9)))
emit(serialize(path(100, lambda i: r.choice((1, 2)))))
