import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(124)
N = 3 * 10**4


def random_tree(n, lo=-1000, hi=1000, null_prob=0.3):
    return tree_level_order([r.randint(lo, hi) for _ in range(n)], r, null_prob)


def spine(vals, side):
    """A list-shaped tree; side is "left", "right" or "zigzag"."""
    out = [vals[0]]
    for i, v in enumerate(vals[1:]):
        go_left = side == "left" or (side == "zigzag" and i % 2 == 0)
        out += [v, None] if go_left else [None, v]
    return out


def vee(left_vals, right_vals, top):
    """A root with a long left-leaning chain and a long right-leaning chain below it."""
    out = [top, left_vals[0], right_vals[0]]
    for a, b in zip(left_vals[1:], right_vals[1:]):
        out += [a, None, None, b]
    return out


# Edge cases.
emit([-3])
emit([0])
emit([1000])
emit([-1000])
emit([2, -1])
emit([-2, -1])
emit([-1, -2, -3])
emit([1, -2, 3])
emit([5, 4, 8, 11, None, 13, 4, 7, 2, None, None, None, 1])
emit([-1000] * 15)
emit([1000] * 15)
emit([10, -5, -5, 20, 20, 20, 20])
emit([-10, 30, 40, -100, -100, -100, -100])
emit(spine([1, -1, 1, -1, 1, -1, 1], "left"))
emit(spine([3, -10, 4, 5], "right"))
# Small random cases across value ranges.
for n in range(2, 30):
    emit(random_tree(n))
for n in range(2, 12):
    emit(random_tree(n, -1000, -1))
for _ in range(25):
    lo, hi = r.choice([(-1000, 1000), (-50, 30), (-1000, 0), (0, 1000), (-3, 3)])
    emit(random_tree(r.randint(30, 2000), lo, hi, r.choice([0.1, 0.3, 0.45])))
# Large cases at the limits: deep chains make re-computing branch gains quadratic.
emit(spine([1000] * N, "left"))
emit(spine([r.randint(-1000, 1000) for _ in range(N)], "right"))
emit(spine([r.randint(-200, 1000) for _ in range(N)], "zigzag"))
emit(spine([-1000] * (N - 1) + [-1], "right"))
half = N // 2
emit(vee([r.randint(-100, 1000) for _ in range(half - 1)], [r.randint(-100, 1000) for _ in range(half)], -1000))
emit(tree_level_order([r.randint(-1000, 1000) for _ in range(N)]))
for _ in range(2):
    emit(random_tree(N, null_prob=0.4))
