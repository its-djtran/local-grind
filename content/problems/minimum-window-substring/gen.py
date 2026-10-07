import string
import sys
from collections import Counter

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(76)
LETTERS = string.ascii_letters
N = 10**5


def min_windows(s, t):
    """Distinct strings among all shortest good windows (empty set if none exist)."""
    need = Counter(t)
    missing, left = len(t), 0
    found = []
    for right, ch in enumerate(s):
        if need[ch] > 0:
            missing -= 1
        need[ch] -= 1
        while missing == 0:
            found.append((right - left + 1, left))
            need[s[left]] += 1
            if need[s[left]] > 0:
                missing += 1
            left += 1
    if not found:
        return set()
    best = min(length for length, _ in found)
    return {s[i:i + best] for length, i in found if length == best}


def valid(s, t):
    return len(min_windows(s, t)) <= 1


def random_case(m, n, alphabet):
    while True:
        s, t = random_string(r, m, alphabet), random_string(r, n, alphabet)
        if valid(s, t):
            return s, t


def planted(m, background, t, spread):
    """A long background without some letter of t, with t's letters scattered in one region."""
    while True:
        s = list(random_string(r, m, background))
        start = r.randint(0, m - spread)
        for ch, pos in zip(t, r.sample(range(start, start + spread), len(t))):
            s[pos] = ch
        s = "".join(s)
        if valid(s, t) and min_windows(s, t):
            return s, t


cases = [
    ("a", "b"),
    ("ab", "b"),
    ("ab", "a"),
    ("ab", "A"),
    ("aa", "aa"),
    ("bba", "ab"),
    ("abc", "cba"),
    ("abc", "abcd"),
    ("cabwefgewcwaefgcf", "cae"),
    ("aaflslflsldkalskaaa", "aaa"),
    ("bdab", "ab"),
    ("acbbaca", "aba"),
    ("ADOBECODEBANCZ", "Z"),
    ("xyzXYZ", "zX"),
]
for s, t in cases:
    assert valid(s, t)
    emit(s, t)
for _ in range(25):
    emit(*random_case(r.randint(1, 12), r.randint(1, 4), r.choice(["ab", "abc", "aAb"])))
for _ in range(20):
    emit(*random_case(r.randint(10, 300), r.randint(1, 8), r.choice(["abcd", "abcdefgh", LETTERS])))
for _ in range(5):
    emit(*random_case(r.randint(1000, 5000), r.randint(5, 40), LETTERS))
# Large inputs: growing a window from every start is quadratic on these.
big = [
    ("a" * N, "a" * N),
    ("a" * N, "a" * (N - 1) + "b"),
    (random_string(r, N, string.ascii_lowercase), "Z"),
    (random_string(r, N, "abc"), "abcd"),
    ("b" * (N // 2) + "a" + "c" * (N // 2 - 1), "ab"),
    ("A" + "b" * (N - 2) + "C", "CA"),
    planted(N, string.ascii_lowercase.replace("q", ""), "qwerty", 40),
    planted(N, "abcdeABCDE", "XYZxyzaA", 200),
    planted(N, string.ascii_uppercase[:-1], "Z" + "ABCDEFGH" * 4, 2000),
    planted(N, LETTERS.replace("m", ""), "m" + LETTERS.replace("m", ""), 5000),
]
for s, t in big:
    assert valid(s, t)
    emit(s, t)
