import string
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(647)
N = 1000
LOWER = string.ascii_lowercase


def brute(s):
    return sum(s[i:j] == s[i:j][::-1] for i in range(len(s)) for j in range(i + 1, len(s) + 1))


def expand(s):
    n, count = len(s), 0
    for c in range(2 * n - 1):
        lo, hi = c // 2, c // 2 + c % 2
        while lo >= 0 and hi < n and s[lo] == s[hi]:
            count += 1
            lo -= 1
            hi += 1
    return count


def put(s):
    assert 1 <= len(s) <= N and set(s) <= set(LOWER)
    if len(s) <= 80:
        assert brute(s) == expand(s)
    emit(s)


def palindrome(n, alphabet):
    half = random_string(r, n // 2, alphabet)
    return half + (r.choice(alphabet) if n % 2 else "") + half[::-1]


for s in ["a", "z", "ab", "aa", "aaa", "abc", "aba", "abba", "abab", "racecar", "abcba",
          "aabbaa", "abcdefghijklmnopqrstuvwxyz", "zzzzzzzzzz", "fdsklf", "leetcode", "noonabbad"]:
    put(s)
for n in range(2, 16):
    put(random_string(r, n, "ab"))
    put(random_string(r, n, "abc"))
for _ in range(15):
    put(random_string(r, r.randint(16, 300), r.choice(["ab", "abc", "abcde", LOWER])))
for _ in range(5):
    put(palindrome(r.randint(16, 300), r.choice(["ab", LOWER])))
# Large cases at the length limit.
put("a" * N)
put("ab" * (N // 2))
put("abc" * (N // 3) + "a")
put("a" * 500 + "b" * 500)
put(palindrome(N, "ab"))
put(palindrome(N, LOWER))
put(LOWER * (N // 26) + LOWER[: N % 26])
for _ in range(16):
    put(random_string(r, N, r.choice(["ab", "abc", "abcd", LOWER])))
