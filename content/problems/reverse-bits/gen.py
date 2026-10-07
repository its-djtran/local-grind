import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(190)
MAX = 2**31 - 2


def brute(n):
    return int(format(n, "032b")[::-1], 2)


def ref(n):
    out = 0
    for _ in range(32):
        out = (out << 1) | (n & 1)
        n >>= 1
    return out


tests = [0, 2, 4, 6, 8, 10, 16, 2**30, 2**30 + 2, 2**16, 2**15, 2**31 - 2, 2**31 - 4, 2**30 - 2,
         0x55555554, 0x2AAAAAAA, 0x0F0F0F0E, 0x7FFF0000, 0x0000FFFE, 0x12345678, 43261596 * 2]
for _ in range(25):
    tests.append(2 * r.randint(0, 500))
for _ in range(35):
    tests.append(2 * r.randint(0, MAX // 2))
# Values confined to the low 16 bits move entirely into the high half.
for _ in range(4):
    half = r.randint(0, 2**15 - 1) << 1
    tests.append(half)
for n in tests:
    assert 0 <= n <= MAX and n % 2 == 0
    assert brute(n) == ref(n) and brute(n) < 2**31
    emit(n)
