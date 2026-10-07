import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(23)
V = 10**4
TOTAL = 10**4
MAXLEN = 500


def sorted_list(n, lo=-V, hi=V):
    return sorted(r.randint(lo, hi) for _ in range(n))


def case(lengths, lo=-V, hi=V):
    assert len(lengths) <= 10**4 and sum(lengths) <= TOTAL
    assert all(0 <= n <= MAXLEN for n in lengths)
    return [sorted_list(n, lo, hi) for n in lengths]


emit([[1]])
emit([[], []])
emit([[], [1], []])
emit([[2], [1]])
emit([[1, 2, 3]])
emit([[-V, V], [0]])
emit([[5, 5, 5], [5, 5], [5]])
emit([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
emit([[7, 8, 9], [4, 5, 6], [1, 2, 3]])
emit([[0]] * 10)
emit([[]] * 50)
emit([list(range(MAXLEN))])

# Small random cases with empty lists mixed in and many ties.
for _ in range(25):
    k = r.randint(1, 8)
    emit(case([r.randint(0, 6) for _ in range(k)], -10, 10))
for _ in range(20):
    k = r.randint(1, 60)
    emit(case([r.randint(0, 40) for _ in range(k)]))

# Large cases. Many short lists make folding one list at a time quadratic.
for _ in range(4):
    emit(case([1] * TOTAL))
emit(case([1] * TOTAL, -5, 5))
emit([[TOTAL - i] for i in range(TOTAL)])
emit(case([2] * (TOTAL // 2)))
emit(case([MAXLEN] * (TOTAL // MAXLEN)))
lengths, left = [], TOTAL
while len(lengths) < 10**4 and left:
    n = min(left, r.choice([0, 0, 1, 2, 3, 5]))
    lengths.append(n)
    left -= n
emit(case(lengths))
