import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(67)
MAX = 10**4


def binary(n):
    """A random binary string of length n without leading zeros."""
    if n == 1:
        return r.choice("01")
    return "1" + "".join(r.choice("01") for _ in range(n - 1))


def column_add(a, b):
    i, j, carry, out = len(a) - 1, len(b) - 1, 0, []
    while i >= 0 or j >= 0 or carry:
        total = carry + (int(a[i]) if i >= 0 else 0) + (int(b[j]) if j >= 0 else 0)
        out.append(str(total % 2))
        carry, i, j = total // 2, i - 1, j - 1
    return "".join(reversed(out))


def check_and_emit(a, b):
    for s in (a, b):
        assert 1 <= len(s) <= MAX and set(s) <= {"0", "1"}
        assert s == "0" or s[0] == "1"
    assert column_add(a, b) == bin(int(a, 2) + int(b, 2))[2:]
    emit(a, b)


fixed = [
    ("0", "0"), ("0", "1"), ("1", "0"), ("1", "1"), ("10", "1"), ("1", "111"), ("111", "111"),
    ("1111", "1"), ("100", "110010"), ("0", "101"), ("1000", "1000"), ("101", "10"),
    ("1" * 20, "1"), ("1" * 20, "1" * 20), ("1" + "0" * 19, "1" + "0" * 19),
]
for a, b in fixed:
    check_and_emit(a, b)
for _ in range(30):
    check_and_emit(binary(r.randint(1, 12)), binary(r.randint(1, 12)))
for _ in range(15):
    check_and_emit(binary(r.randint(1, 500)), binary(r.randint(1, 500)))
for _ in range(4):
    check_and_emit(binary(r.randint(1000, MAX)), binary(r.randint(1, MAX)))
# Maximal inputs, including a carry that ripples through every column.
check_and_emit(binary(MAX), binary(MAX))
check_and_emit("1" * MAX, "1")
check_and_emit("1" * MAX, "1" * MAX)
check_and_emit("1" + "0" * (MAX - 1), "0")
check_and_emit("1", "1" * MAX)
