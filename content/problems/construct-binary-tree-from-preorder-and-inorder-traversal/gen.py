import sys
from collections import deque

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(105)
N = 3000
VALUES = range(-3000, 3001)


class T:
    __slots__ = ("val", "left", "right")

    def __init__(self, val, left=None, right=None):
        self.val, self.left, self.right = val, left, right


def build(arr):
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


def traversals(arr):
    """(preorder, inorder) of a level-order tree, computed iteratively."""
    root = build(arr)
    pre, stack = [], [root]
    while stack:
        node = stack.pop()
        if node:
            pre.append(node.val)
            stack += [node.right, node.left]
    ino, stack, node = [], [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        ino.append(node.val)
        node = node.right
    return pre, ino


def case(arr):
    pre, ino = traversals(arr)
    assert 1 <= len(pre) <= N and len(set(pre)) == len(pre) == len(ino) and set(pre) == set(ino)
    emit(pre, ino)


def distinct(n):
    return r.sample(VALUES, n)


def random_tree(n, null_prob=0.3):
    return tree_level_order(distinct(n), r, null_prob)


def path(vals, sides):
    out = [vals[0]]
    for v, s in zip(vals[1:], sides):
        out += [v, None] if s == "L" else [None, v]
    return out


# Edge cases.
case([-1])
case([3000])
case([1, 2])
case([1, None, 2])
case([0, -3000, 3000])
case([2, 1, 3])
case([1, 2, None, 3])
case([1, None, 2, None, 3])
case([1, 2, None, None, 3])
case([1, None, 2, 3])
case([4, 2, 6, 1, 3, 5, 7])
case(path(distinct(8), "LRLRLRL"))
case(list(range(1, 32)))
# Small random trees.
for n in range(2, 35):
    case(random_tree(n))
for _ in range(25):
    case(random_tree(r.randint(35, N), r.choice([0.1, 0.3, 0.45])))
# Large cases at the limit: skewed trees make search-and-slice quadratic.
case(path(distinct(N), "L" * (N - 1)))
case(path(distinct(N), "R" * (N - 1)))
case(path(distinct(N), "LR" * (N // 2)))
case(path(list(range(3000, 0, -1)), "L" * (N - 1)))
case(path(list(range(-3000, 0)), "R" * (N - 1)))
case(path(distinct(N), "".join(r.choice("LR") for _ in range(N - 1))))
case(path(distinct(N), "".join("L" if i % 9 else "R" for i in range(N - 1))))
for k in (2, 3, 5, 50):
    case(path(distinct(N), "".join("R" if i % k == 0 else "L" for i in range(N - 1))))
    case(path(distinct(N), "".join("L" if i % k == 0 else "R" for i in range(N - 1))))
case(tree_level_order(distinct(N)))
case(random_tree(N, 0.4))
