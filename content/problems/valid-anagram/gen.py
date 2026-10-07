import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(242)
N = 5 * 10**4


def shuffled(s):
    chars = list(s)
    r.shuffle(chars)
    return "".join(chars)


def near_miss(s):
    """Same length, one letter changed: counts differ by exactly one in two places."""
    chars = list(s)
    i = r.randrange(len(chars))
    chars[i] = r.choice([c for c in "abcdefghijklmnopqrstuvwxyz" if c != chars[i]])
    return shuffled("".join(chars))


emit("a", "a")
emit("a", "b")
emit("a", "aa")
emit("ab", "a")
emit("ab", "ba")
emit("aacc", "ccac")
emit("aabb", "abab")
emit("abc", "abcd")
emit("listen", "silent")
emit("abcdefghijklmnopqrstuvwxyz", "zyxwvutsrqponmlkjihgfedcba")
emit("zzzz", "zzzz")
emit("aab", "abb")
for _ in range(15):
    s = random_string(r, r.randint(1, 10), "abc")
    emit(s, shuffled(s))
for _ in range(15):
    s = random_string(r, r.randint(1, 10), "abc")
    emit(s, random_string(r, len(s) + r.choice([0, 0, 1, -1]) if len(s) > 1 else 1, "abc"))
for _ in range(10):
    s = random_string(r, r.randint(10, 1000))
    emit(s, r.choice([shuffled(s), near_miss(s)]))
emit("a" * N, "a" * N)
emit("a" * N, "a" * (N - 1) + "b")
emit("a" * N, "a" * (N - 1))
emit(random_string(r, N), random_string(r, N - 7))
# Long anagram pairs: cross-off-one-by-one does ~n^2 work on these.
for _ in range(6):
    s = random_string(r, N)
    emit(s, shuffled(s))
for _ in range(2):
    s = random_string(r, N)
    emit(s, near_miss(s))
