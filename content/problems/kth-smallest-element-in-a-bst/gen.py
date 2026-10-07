import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import bst_level_order, emit, rng

r = rng(230)
VMAX = 10**4

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


def case(root, n, k):
    assert 1 <= k <= n
    emit(serialize(root), k)


emit([0], 1)
emit([VMAX], 1)
emit([1, 0], 1)
emit([1, 0], 2)
emit([0, None, VMAX], 2)
emit([2, 1, 3], 2)
emit([2, 1, 3], 3)
emit([5, 3, 6, 2, 4, None, None, 1], 6)
emit([5, 3, 6, 2, 4, None, None, 1], 4)
emit([3, 1, 4, None, 2], 4)

for n in range(2, 30):
    values = r.sample(range(0, 60), n)
    case(random_bst(values), n, r.randint(1, n))
for _ in range(20):
    n = r.randint(30, 2000)
    case(random_bst(r.sample(range(0, VMAX + 1), n)), n, r.randint(1, n))

N = 10**4
# Degenerate trees: the k-th smallest can sit at the far end of a long path.
case(chain(list(range(N)), 2), N, N)
case(chain(list(range(N)), 2), N, 1)
case(chain(list(range(N, 0, -1)), 1), N, N)
case(chain(list(range(N, 0, -1)), 1), N, 1)
case(chain(list(range(N, 0, -1)), 1), N, 5000)
values = sorted(r.sample(range(0, VMAX + 1), N))
emit(bst_level_order(values), N)
emit(bst_level_order(values), 1)
emit(bst_level_order(values), 7777)
for k in (1, N // 2, N):
    case(random_bst(r.sample(range(0, VMAX + 1), N)), N, k)
