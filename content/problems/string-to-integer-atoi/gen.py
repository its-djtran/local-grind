import re
import string
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(8)
ALPHABET = string.ascii_letters + string.digits + " +-."
INT_MAX, INT_MIN = 2**31 - 1, -(2**31)


def reference(s):
    m = re.match(r" *([+-]?[0-9]+)", s)
    return max(INT_MIN, min(INT_MAX, int(m.group(1)))) if m else 0


def solution(s):
    i, n = 0, len(s)
    while i < n and s[i] == " ":
        i += 1
    sign = 1
    if i < n and s[i] in "+-":
        sign = -1 if s[i] == "-" else 1
        i += 1
    value = 0
    while i < n and s[i].isdigit():
        d = ord(s[i]) - ord("0")
        if value > (INT_MAX - d) // 10:
            return INT_MAX if sign == 1 else INT_MIN
        value = value * 10 + d
        i += 1
    return sign * value


def put(s):
    assert len(s) <= 200 and all(c in ALPHABET for c in s)
    assert reference(s) == solution(s), s
    emit(s)


def tail(n):
    return "".join(r.choice(ALPHABET) for _ in range(n))


def structured():
    """Spaces, then a sign, zeros, digits, then junk; each part randomly present or tweaked."""
    s = " " * r.choice([0, 0, 1, 3, r.randint(0, 20)])
    s += r.choice(["", "", "+", "-", "-", "+-", "-+", "--", " -", "."])
    s += "0" * r.choice([0, 0, 0, 1, 5, r.randint(0, 30)])
    s += "".join(r.choice(string.digits) for _ in range(r.choice([0, 1, 2, 5, 9, 10, 11, 12, 20])))
    s += tail(r.choice([0, 0, 1, 3, 10]))
    return s[:200]


for s in ["", " ", "     ", "+", "-", "+-12", "-+12", "++1", "--1", "+1", "-1", "0", "-0", "+0",
          "  +0 123", "00000-42a1234", "-91283472332", "91283472332", "2147483647", "2147483648",
          "-2147483647", "-2147483648", "-2147483649", "21474836460", "-21474836480", "2147483646",
          "-2147483646", "4294967296", "9223372036854775808", ".1", "3.14159", "-3.9", "   +004500",
          "-0012a42", "  0000000000012345678", "-5-", "1 2", " - 1", "+ 1", "abc", "A1", "e10",
          "-000000000000001", "0000000000000000000000000000002147483648", " 1a", "-.5", "+.5"]:
    put(s)
put("0" * 199 + "7")
put("-" + "0" * 189 + "2147483648")
put(" " * 199 + "1")
put(" " * 200)
put("9" * 200)
put("-" + "1" * 199)
put("1" * 10 + "." + "1" * 189)
for _ in range(32):
    put(structured())
for _ in range(6):
    put(tail(r.randint(1, 200)))
