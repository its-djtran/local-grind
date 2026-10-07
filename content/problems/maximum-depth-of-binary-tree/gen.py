import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng, tree_level_order

r = rng(104)


def random_tree(n, null_prob):
    return tree_level_order([r.randint(-100, 100) for _ in range(n)], r, null_prob)


def spine(n, side):
    """A degenerate tree that is one long path, leaning left or right."""
    out = [0]
    for i in range(1, n):
        out += [i % 100, None] if side == "left" else [None, i % 100]
    return out


emit([])
emit([0])
emit([1, 2])
emit([1, None, 2])
emit([1, 2, 3, 4, 5, 6, 7])
emit(spine(5, "left"))
emit(spine(5, "right"))
for n in range(2, 25):
    emit(random_tree(n, 0.3))
for _ in range(25):
    emit(random_tree(r.randint(25, 2000), r.choice([0.1, 0.3, 0.45])))
emit(spine(10**4, "left"))
emit(spine(10**4, "right"))
emit(tree_level_order([r.randint(-100, 100) for _ in range(10**4)]))
for _ in range(3):
    emit(random_tree(10**4, 0.4))
