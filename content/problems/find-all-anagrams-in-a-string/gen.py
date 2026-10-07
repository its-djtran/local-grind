import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(438)
N = 3 * 10**4


def shuffled(s):
    chars = list(s)
    r.shuffle(chars)
    return "".join(chars)


def planted(n, p, copies, alphabet):
    """Random text with `copies` shuffled copies of p written over it at random places."""
    s = list(random_string(r, n, alphabet))
    for _ in range(copies):
        i = r.randint(0, n - len(p))
        s[i:i + len(p)] = shuffled(p)
    return "".join(s)


emit("a", "a")
emit("a", "b")
emit("a", "ab")
emit("ab", "abc")
emit("aa", "a")
emit("aaaaa", "aa")
emit("baa", "aa")
emit("abacbabc", "abc")
emit("ab", "ba")
emit("abcd", "dcba")
emit("af", "be")
emit("abcdefghijklmnopqrstuvwxyz", "zyx")
for _ in range(25):
    p = random_string(r, r.randint(1, 4), "ab")
    emit(random_string(r, r.randint(1, 15), "ab"), p)
for _ in range(15):
    p = random_string(r, r.randint(2, 20), r.choice(["abc", "abcdef"]))
    emit(planted(r.randint(len(p), 400), p, r.randint(0, 5), "abcdef"), p)
emit("a" * N, "a")
emit("a" * N, "a" * N)
emit("a" * N, "a" * (N - 1) + "b")
emit(random_string(r, 100), random_string(r, N))
emit("ab" * (N // 2), "ab" * 100)
# A long p with many windows: rebuilding each window's counts costs ~n*m.
for _ in range(4):
    p = random_string(r, r.randint(8000, 15000), "ab")
    emit(planted(N, p, 6, "ab"), p)
for _ in range(2):
    p = random_string(r, r.randint(5000, 15000))
    emit(planted(N, p, 3, "abcdefghijklmnopqrstuvwxyz"), p)
