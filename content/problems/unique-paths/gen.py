import math
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(62)
CAP = 2 * 10**9


def check(m, n):
    assert 1 <= m <= 100 and 1 <= n <= 100 and math.comb(m + n - 2, m - 1) <= CAP
    emit(m, n)


# Edge cases.
for m, n in [(1, 1), (1, 2), (2, 1), (2, 2), (1, 100), (100, 1), (2, 100), (100, 2),
             (3, 100), (100, 3), (7, 3), (3, 3), (10, 10), (4, 4), (5, 9)]:
    check(m, n)

# Small random cases.
for _ in range(25):
    check(r.randint(1, 10), r.randint(1, 10))

# Random cases anywhere in range, retried until the answer respects the 2 * 10^9 cap.
for _ in range(15):
    while True:
        m, n = r.randint(1, 100), r.randint(1, 100)
        if math.comb(m + n - 2, m - 1) <= CAP:
            break
    check(m, n)

# Largest answers: the plain recursion makes one call per route, i.e. billions of calls.
largest = sorted(((math.comb(m + n - 2, m - 1), m, n)
                  for m in range(1, 101) for n in range(1, 101)
                  if math.comb(m + n - 2, m - 1) <= CAP), reverse=True)
seen = set()
for _, m, n in largest:
    if len(seen) == 8:
        break
    if (m, n) not in seen:
        seen.add((m, n))
        check(m, n)
check(17, 17)
check(23, 12)
