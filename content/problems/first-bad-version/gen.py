import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(278)
MAX = 2**31 - 1


def check_and_emit(n, bad):
    assert 1 <= bad <= n <= MAX
    emit(n, bad)


fixed = [(1, 1), (2, 1), (2, 2), (3, 1), (3, 2), (3, 3), (4, 3), (10, 10), (10, 1), (100, 51),
         (MAX, 1), (MAX, 2), (MAX, MAX), (MAX, MAX - 1), (MAX, 2**30), (MAX, 2**30 + 1),
         (2**30, 2**30), (2**31 - 2, 2**31 - 2), (1702766719, 1702766719), (2126753390, 1702766719)]
for n, bad in fixed:
    check_and_emit(n, bad)
for n in range(4, 12):
    for bad in (1, (n + 1) // 2, n):
        check_and_emit(n, bad)
for _ in range(20):
    n = r.randint(1, 1000)
    check_and_emit(n, r.randint(1, n))
for _ in range(15):
    n = r.randint(1, MAX)
    check_and_emit(n, r.randint(1, n))
# Large n with a late first bad version: a linear scan would need billions of calls.
for _ in range(10):
    n = r.randint(MAX - 10**6, MAX)
    check_and_emit(n, r.randint(n - 10**6, n))
