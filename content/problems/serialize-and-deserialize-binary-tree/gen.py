import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(297)
N = 10**4


def random_tree(n, null_prob=0.3, lo=-1000, hi=1000):
    return tree_level_order([r.randint(lo, hi) for _ in range(n)], r, null_prob)


def path(vals, sides):
    """A single root-to-leaf path; sides[i] says whether node i+1 hangs left or right."""
    out = [vals[0]]
    for v, s in zip(vals[1:], sides):
        out += [v, None] if s == "L" else [None, v]
    return out


def rand_vals(n):
    return [r.randint(-1000, 1000) for _ in range(n)]


# Edge cases: empty, single nodes at the value bounds, shapes that differ only by side.
emit([])
emit([0])
emit([-1000])
emit([1000])
emit([1, 2])
emit([1, None, 2])
emit([-1, -2, -3])
emit([1, 23])
emit([12, 3])
emit([0, 0, 0, None, 0, 0])
emit([-1000, 1000, -1000, 1000])
emit([5, 4, 7, 3, None, 2, None, -1, None, 9])
emit(path([1, 2, 3, 4, 5, 6], "LRRLL"))
emit(list(range(-7, 8)))
# Small random trees.
for n in range(2, 35):
    emit(random_tree(n))
for _ in range(25):
    emit(random_tree(r.randint(35, 3000), r.choice([0.1, 0.3, 0.45])))
# Large cases at the limits, including list-shaped trees 10^4 levels deep.
emit(path(rand_vals(N), "L" * (N - 1)))
emit(path(rand_vals(N), "R" * (N - 1)))
emit(path(rand_vals(N), "LR" * (N // 2)))
emit(path([-1000] * N, "".join(r.choice("LR") for _ in range(N - 1))))
emit(tree_level_order(rand_vals(N)))
emit(random_tree(N, 0.4))
emit(random_tree(N, 0.1))
