import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import bst_level_order, emit, rng

r = rng(98)
LO, HI = -(2**31), 2**31 - 1

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


def nodes_of(root):
    out, stack = [], [root]
    while stack:
        node = stack.pop()
        if node:
            out.append(node)
            stack += [node[1], node[2]]
    return out


def is_bst(root):
    stack = [(root, -float("inf"), float("inf"))]
    while stack:
        node, lo, hi = stack.pop()
        if node is None:
            continue
        if not lo < node[0] < hi:
            return False
        stack += [(node[1], lo, node[0]), (node[2], node[0], hi)]
    return True


def brute_is_bst(root):
    """Direct use of the definition, for cross-checking small trees."""
    def vals(n):
        return [] if n is None else vals(n[1]) + [n[0]] + vals(n[2])

    if root is None:
        return True
    if any(v >= root[0] for v in vals(root[1])) or any(v <= root[0] for v in vals(root[2])):
        return False
    return brute_is_bst(root[1]) and brute_is_bst(root[2])


def random_bst(values):
    """Inserts values (distinct) in the given order into an unbalanced BST."""
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


def random_shape(n, lo, hi):
    """A random binary tree shape with random (possibly repeated) values: rarely a BST."""
    root = [r.randint(lo, hi), None, None]
    all_nodes = [root]
    while len(all_nodes) < n:
        parent = r.choice(all_nodes)
        side = r.choice((1, 2))
        if parent[side] is None:
            parent[side] = [r.randint(lo, hi), None, None]
            all_nodes.append(parent[side])
    return root


def corrupt(root, lo, hi):
    """Changes one node's value so the tree stops being a BST."""
    ns = nodes_of(root)
    while True:
        node = r.choice(ns)
        old = node[0]
        choice = r.random()
        if choice < 0.4:
            other = r.choice(ns)
            node[0] = other[0]  # creates a duplicate (or is a no-op)
        elif choice < 0.8:
            node[0] = old + r.choice((-1, 1)) * r.randint(1, max(1, (hi - lo) // 4))
            node[0] = min(max(node[0], LO), HI)
        else:
            node[0] = r.randint(lo, hi)
        if not is_bst(root):
            return root
        node[0] = old


def chain(values, side):
    root = cur = [values[0], None, None]
    for v in values[1:]:
        nxt = [v, None, None]
        cur[side] = nxt
        cur = nxt
    return root


def zigzag(n):
    """A path that alternates right and left turns while staying a valid BST."""
    small, big = 0, n
    root = cur = [small, None, None]
    for i in range(1, n):
        if i % 2:
            nxt, side = [big, None, None], 2
            big -= 1
        else:
            small += 1
            nxt, side = [small, None, None], 1
        cur[side] = nxt
        cur = nxt
    return root


def emit_tree(root):
    emit(serialize(root))


# Edge cases.
emit([1])
emit([HI])
emit([LO])
emit([LO, None, HI])
emit([HI, LO])
emit([HI, HI])
emit([LO, None, LO])
emit([1, 1])
emit([1, None, 1])
emit([2, 2, 2])
emit([5, 4, 6, None, None, 3, 7])
emit([3, 1, 5, 0, 2, 4, 6])
emit([10, 5, 15, None, None, 6, 20])
emit([32, 26, 47, 19, None, None, 56, None, 27])
emit([0, -1])
emit([0, None, -1])
emit([3, None, 30, 10, None, None, 15, None, 45])
emit([1, None, 2, None, 3, None, 4])
emit([4, 3, None, 2, None, 1])

# Small random cases: half valid, half corrupted, plus random shapes. Each is cross-checked.
small = []
for _ in range(20):
    n = r.randint(2, 15)
    values = r.sample(range(-20, 21), n)
    small.append(random_bst(values))
for _ in range(16):
    n = r.randint(2, 15)
    small.append(corrupt(random_bst(r.sample(range(-20, 21), n)), -20, 20))
for _ in range(6):
    small.append(random_shape(r.randint(2, 8), 0, 5))
for t in small:
    assert is_bst(t) == brute_is_bst(t)
    emit_tree(t)

# Medium random cases over the full value range.
for _ in range(6):
    n = r.randint(100, 2000)
    emit_tree(random_bst(r.sample(range(LO, HI), n)))
for _ in range(6):
    n = r.randint(100, 2000)
    emit_tree(corrupt(random_bst(r.sample(range(LO, HI), n)), LO, HI))

# Large cases: deep, valid (or violated only at the bottom) trees defeat quadratic approaches.
N = 10**4
emit_tree(chain(list(range(N)), 2))
emit_tree(chain(list(range(N, 0, -1)), 1))
emit_tree(chain([LO + i * 400_000 for i in range(N)], 2))
emit_tree(zigzag(N))
assert is_bst(zigzag(N))
emit(bst_level_order(sorted(r.sample(range(LO, HI), N))))
emit_tree(random_bst(r.sample(range(LO, HI), N)))
bad = chain(list(range(N)), 2)
cur = bad
while cur[2]:
    cur = cur[2]
cur[0] = N - 3  # duplicate of an ancestor at the very bottom
emit_tree(bad)
bad = chain(list(range(N, 0, -1)), 1)
cur = bad
while cur[1]:
    cur = cur[1]
cur[1] = [cur[0] + 1, None, None]
emit_tree(bad)
