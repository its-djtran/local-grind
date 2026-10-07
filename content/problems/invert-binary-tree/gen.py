import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(226)


def random_tree(n, null_prob=0.3, lo=-100, hi=100):
    return tree_level_order([r.randint(lo, hi) for _ in range(n)], r, null_prob)


def path(n, sides):
    """A single root-to-leaf path; sides[i] says whether node i+1 hangs left or right."""
    out = [0]
    for i in range(1, n):
        out += [i % 201 - 100, None] if sides[i - 1] == "L" else [None, i % 201 - 100]
    return out


emit([])
emit([1])
emit([-100])
emit([1, 2])
emit([1, None, 2])
emit([1, 1, 1])
emit([3, 1, 2])
emit([1, 2, None, 3])
emit([1, None, 2, None, 3])
emit([1, 2, 3, 4, None, None, 5])
emit([0, -100, 100, -50, 50, -25, 25])
emit(path(100, "L" * 99))
emit(path(100, "R" * 99))
emit(path(100, "LR" * 50))
emit(path(100, "".join(r.choice("LR") for _ in range(99))))
emit(list(range(-50, 50)))
emit([7] * 100)
for n in range(2, 30):
    emit(random_tree(n))
for _ in range(30):
    emit(random_tree(r.randint(30, 100), r.choice([0.1, 0.3, 0.45])))
for _ in range(4):
    emit(random_tree(100, r.choice([0.0, 0.3])))
