import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(191)
MAX = 2**31 - 1


def brute(n):
    return bin(n).count("1")


tests = [1, 2, 3, 4, 7, 8, 15, 16, 255, 256, 1023, 1024, 2**30, 2**30 + 1, 2**30 - 1, MAX, MAX - 1,
         2**31 - 2**16, 0x55555555, 0x2AAAAAAA, 0x0F0F0F0F, 0x70F0F0F0]
tests += [1 << i for i in range(31) if (1 << i) not in tests][:10]
for _ in range(25):
    tests.append(r.randint(1, 1000))
for _ in range(25):
    tests.append(r.randint(1, MAX))
# Numbers with a chosen density of set bits.
for k in (1, 5, 16, 25, 30):
    for _ in range(2):
        bits = r.sample(range(31), k)
        tests.append(sum(1 << b for b in bits))
for n in tests:
    assert 1 <= n <= MAX
    emit(n)
