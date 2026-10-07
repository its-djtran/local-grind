import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(102)
N = 2000


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


# Edge cases.
emit([])
emit([0])
emit([-1000])
emit([1, 2])
emit([1, None, 2])
emit([1, 2, 3])
emit([3, 2, 1, 6, 5, 4, 7])
emit([1, 2, 3, 4, None, None, 5])
emit([1, None, 2, 3])
emit([1, 2, None, None, 3, 4])
emit([5, 5, 5, 5, 5, 5, 5, 5])
emit(path([1, 2, 3, 4, 5], "LRLR"))
emit(list(range(1, 32)))
# Small random trees.
for n in range(2, 35):
    emit(random_tree(n))
for _ in range(25):
    emit(random_tree(r.randint(35, N), r.choice([0.1, 0.3, 0.45])))
# Large cases at the limit: deep trees make level-by-level rescans quadratic.
emit(path(rand_vals(N), "L" * (N - 1)))
emit(path(rand_vals(N), "R" * (N - 1)))
emit(path(rand_vals(N), "LR" * (N // 2)))
emit(path(rand_vals(N), "".join(r.choice("LR") for _ in range(N - 1))))
emit(path(list(range(-1000, 1000)), "RL" * (N // 2)))
# A caterpillar: a long left-leaning spine where every spine node also has a right leaf.
cat = [r.randint(-1000, 1000)]
for _ in range(N // 2 - 1):
    cat += [r.randint(-1000, 1000), r.randint(-1000, 1000)]
emit(cat)
emit(path(rand_vals(N), "".join("L" if i % 7 else "R" for i in range(N - 1))))
emit(tree_level_order(rand_vals(N)))
emit(random_tree(N, 0.45))
