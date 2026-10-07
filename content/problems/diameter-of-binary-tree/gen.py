import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(543)
N = 10**4


def val():
    return r.randint(-100, 100)


def random_tree(n, null_prob=0.3):
    return tree_level_order([val() for _ in range(n)], r, null_prob)


def path(n, sides):
    out = [val()]
    for s in sides[: n - 1]:
        out += [val(), None] if s == "L" else [None, val()]
    return out


class T:
    __slots__ = ("val", "left", "right")

    def __init__(self, val, left=None, right=None):
        self.val, self.left, self.right = val, left, right


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


def chain(n, side):
    """A list-shaped subtree of n nodes leaning to one side; returns its top node."""
    top = cur = T(val())
    for _ in range(n - 1):
        nxt = T(val())
        if side == "L":
            cur.left = nxt
        else:
            cur.right = nxt
        cur = nxt
    return top


def offset_vee(short, left_len, right_len):
    """The root has a short left leg; its right child holds two long chains, so the
    longest path bends below the root instead of at it."""
    b = T(val(), chain(left_len, "L"), chain(right_len, "R"))
    return dump(T(val(), chain(short, "L"), b))


# Edge cases.
emit([1])
emit([1, 2])
emit([1, None, 2])
emit([1, 2, 3])
emit([1, 2, None, 3, 4])
emit([1, 2, None, 3, 4, 5, None, None, 6])
emit([4, -7, -3, None, None, -9, -3, 9, -7, -4, None, 6, None, -6, -6, None, None, 0, 6, 5, None, 9, None, None, -1, -4, None, None, None, -2])
emit([1, 2, 3, 4, 5, 6, 7])
emit([0] * 15)
emit(path(6, "LLLLL"))
emit(path(6, "LRLRL"))
emit(offset_vee(1, 3, 4))
emit(offset_vee(2, 5, 6))
# Small random trees.
for n in range(2, 35):
    emit(random_tree(n))
for _ in range(25):
    emit(random_tree(r.randint(35, 3000), r.choice([0.1, 0.3, 0.45])))
# Large cases at the limit: deep trees make re-measuring heights quadratic.
emit(path(N, "L" * N))
emit(path(N, "R" * N))
emit(path(N, "LR" * N))
emit(path(N, "".join(r.choice("LR") for _ in range(N))))
emit(offset_vee(10, 4000, 5000))
emit(tree_level_order([val() for _ in range(N)]))
emit(random_tree(N, 0.4))
