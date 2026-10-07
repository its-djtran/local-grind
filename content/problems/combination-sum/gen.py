import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(39)


def count_combinations(candidates, target):
    """Number of multisets of candidates summing to target (coin-change counting DP)."""
    ways = [1] + [0] * target
    for c in candidates:
        for s in range(c, target + 1):
            ways[s] += ways[s - c]
    return ways[target]


def case(candidates, target):
    assert 1 <= len(candidates) <= 30 and len(set(candidates)) == len(candidates)
    assert all(2 <= c <= 40 for c in candidates) and 1 <= target <= 40
    assert count_combinations(candidates, target) < 150
    emit(candidates, target)


def random_case(n_lo, n_hi, t_lo, t_hi, c_lo=2, c_hi=40):
    while True:
        n = r.randint(n_lo, n_hi)
        candidates = r.sample(range(c_lo, c_hi + 1), min(n, c_hi - c_lo + 1))
        target = r.randint(t_lo, t_hi)
        if count_combinations(candidates, target) < 150:
            return candidates, target


# Edge cases.
case([2], 1)
case([2], 2)
case([2], 40)
case([3], 40)
case([40], 40)
case([39, 40], 1)
case([7, 3, 2], 18)
case([8, 7, 4, 3], 11)
case([2, 3, 6, 7], 1)
case([5, 10], 40)
case([2, 4, 8, 16, 32], 40)
case([40, 39, 38, 37], 40)
case([13, 17, 19, 23, 29], 40)
case(list(range(11, 41)), 40)  # 30 candidates, only pairs and singletons fit

# Small random cases.
for _ in range(30):
    case(*random_case(1, 6, 1, 25, 2, 15))
for _ in range(20):
    case(*random_case(1, 30, 1, 40))

# Large cases: target 40 with small candidates gives the deepest search trees allowed while
# staying under 150 combinations. Brute-force enumeration of ordered sequences explodes here.
best = []
for _ in range(4000):
    n = r.randint(2, 5)
    candidates = sorted(r.sample(range(2, 12), n))
    ways = count_combinations(candidates, 40)
    if ways < 150:
        best.append((ways, candidates))
best.sort(reverse=True)
chosen = []
for ways, candidates in best:
    if candidates not in chosen and 2 in candidates:
        chosen.append(candidates)
    if len(chosen) == 6:
        break
for candidates in chosen:
    case(r.sample(candidates, len(candidates)), 40)
case([2, 3], 40)
case([2, 3, 5], 40) if count_combinations([2, 3, 5], 40) < 150 else case([3, 5, 7], 40)
case([3, 4, 5], 40)
case([2, 5, 7, 11], 40)
