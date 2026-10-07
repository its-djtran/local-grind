import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(338)
MAX = 10**5

tests = list(range(0, 21)) + [31, 32, 63, 64, 127, 128, 255, 256, 1023, 1024, 4095, 65535, 65536]
for _ in range(20):
    tests.append(r.randint(21, 2000))
for _ in range(8):
    tests.append(r.randint(2000, MAX))
tests += [MAX] * 8 + [MAX - 1, 2**16 + 2**15, 99999, 98765]
for n in tests:
    if n <= 3000:
        expected = [bin(i).count("1") for i in range(n + 1)]
        ans = [0] * (n + 1)
        for i in range(1, n + 1):
            ans[i] = ans[i >> 1] + (i & 1)
        assert ans == expected
    emit(n)
