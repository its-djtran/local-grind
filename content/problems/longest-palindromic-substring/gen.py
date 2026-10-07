import string
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(5)
N = 1000
ALNUM = string.ascii_letters + string.digits


def longest_set(s):
    """All distinct palindromic substrings of maximum length (expand around every center)."""
    best, found = 0, set()
    for c in range(2 * len(s) - 1):
        lo, hi = c // 2, c // 2 + c % 2
        while lo >= 0 and hi < len(s) and s[lo] == s[hi]:
            lo -= 1
            hi += 1
        length = hi - lo - 1
        if length > best:
            best, found = length, {s[lo + 1:hi]}
        elif length == best and length > 0:
            found.add(s[lo + 1:hi])
    return found


def brute(s):
    subs = [s[i:j] for i in range(len(s)) for j in range(i + 1, len(s) + 1)]
    pals = [t for t in subs if t == t[::-1]]
    m = max(map(len, pals))
    return {t for t in pals if len(t) == m}


def unique(s):
    return len(longest_set(s)) == 1


def put(s):
    assert 1 <= len(s) <= N and all(c in ALNUM for c in s)
    found = longest_set(s)
    assert len(found) == 1, s
    if len(s) <= 60:
        assert brute(s) == found, s
    emit(s)


def palindrome(n, alphabet):
    half = random_string(r, n // 2, alphabet)
    return half + (r.choice(alphabet) if n % 2 else "") + half[::-1]


def random_unique(n, alphabet):
    while True:
        s = random_string(r, n, alphabet)
        if unique(s):
            return s


def planted(n, p, alphabet, where=None):
    """A random string of length n with a palindrome of length p planted inside."""
    while True:
        pal = palindrome(p, alphabet)
        pos = r.randint(0, n - p) if where is None else where
        s = random_string(r, pos, alphabet) + pal + random_string(r, n - p - pos, alphabet)
        if unique(s):
            return s


for s in ["a", "z", "7", "bb", "aa", "aba", "abb", "bba", "abba", "Aa1aA", "aaaa", "aaab",
          "baaa", "abcba", "abacdfgdcaba", "racecar", "xracecarx1", "0110", "1aA1Aa",
          "abcdcbaxyz", "forgeeksskeegfor", "ccc", "aabbaa"]:
    if unique(s):
        put(s)
for n in range(2, 15):
    put(random_unique(n, "ab"))
    put(random_unique(n, "abc"))
for _ in range(20):
    n = r.randint(15, 300)
    put(planted(n, r.randint(3, min(n, 40)), r.choice(["ab", "abc", "abcde", ALNUM])))
# Large cases at the length limit.
put("a" * N)
put("ab" * (N // 2 - 1) + "a")
put("a" * 499 + "b" + "a" * 500)
put(palindrome(N, ALNUM))
put(planted(N, 12, ALNUM))
put(planted(N, 12, ALNUM, where=0))
put(planted(N, 12, ALNUM, where=N - 12))
put(planted(N, 501, string.ascii_lowercase))
put(planted(N, 30, "abc"))
for _ in range(10):
    put(random_unique(N, r.choice(["ab", "abc", "abcd", string.digits])))
for _ in range(10):
    put(planted(N, r.randint(8, 20), r.choice([string.ascii_lowercase, ALNUM])))
