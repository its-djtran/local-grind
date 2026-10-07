import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(91)
INT_MAX = 2**31 - 1


def ways(s):
    """Independent check: count decodings with a forward table."""
    dp = [0] * (len(s) + 1)
    dp[0] = 1
    for i in range(len(s)):
        if dp[i] == 0:
            continue
        if s[i] != "0":
            dp[i + 1] += dp[i]
            if i + 1 < len(s) and 10 <= int(s[i:i + 2]) <= 26:
                dp[i + 2] += dp[i]
    return dp[len(s)]


def check(s):
    assert 1 <= len(s) <= 100 and s.isdigit() and ways(s) <= INT_MAX
    emit(s)


def token_string(n, p_small, p_zero):
    out = ""
    while len(out) < n:
        x = r.random()
        if x < p_zero:
            out += r.choice(["10", "20", "0"])
        elif x < p_zero + p_small:
            out += r.choice("12")
        else:
            out += r.choice("3456789")
    return out[:n]


def big_answer(n, lo):
    """A length-n string whose decoding count lies in [lo, INT_MAX]."""
    while True:
        s = token_string(n, r.uniform(0.3, 0.75), r.choice([0.0, 0.03]))
        if lo <= ways(s) <= INT_MAX:
            return s


# Edge cases.
for s in ["0", "1", "9", "10", "20", "30", "27", "26", "00", "01", "100", "101", "110",
          "230", "2101", "11106", "12120", "1201234", "301", "2611055971756562", "10011"]:
    check(s)
check("1" * 45)          # Fibonacci(46): the largest all-ones input that fits
check("2" * 45)
check("26" * 30)
check("10" * 50)
check("9" * 100)
check("0" * 100)
check("3" * 56 + "1" * 43 + "0")
check("0" + "1" * 40)

# Small random cases.
for _ in range(30):
    n = r.randint(1, 12)
    s = "".join(r.choice("0112223456789") for _ in range(n))
    check(s)

# Medium random cases.
for _ in range(8):
    s = token_string(r.randint(13, 60), r.uniform(0.2, 0.6), 0.04)
    if ways(s) > INT_MAX:
        s = big_answer(len(s), 1)
    check(s)

# Large cases: answers near 2^31 make the unmemoized recursion explode.
for _ in range(6):
    check(big_answer(100, 10**8))
check("3" + "1" * 44 + "3" * 55)
