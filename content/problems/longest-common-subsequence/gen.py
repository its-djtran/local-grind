import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(1143)
AZ = "abcdefghijklmnopqrstuvwxyz"


def subsequence_of(s):
    return "".join(c for c in s if r.random() < 0.5) or s[0]


# Edge cases.
emit("a", "a")
emit("a", "b")
emit("ab", "ba")
emit("bsbininm", "jmjkbkjkv")
emit("oxcpqrsvwf", "shmtulqrypy")
emit("ezupkr", "ubmrapg")
emit("aaaa", "aa")
emit("abcba", "abcbcba")
emit("z" * 1000, "z")
emit("a" * 1000, "b" * 1000)            # nothing in common
emit(AZ * 38 + "abcdefghijkl", AZ[::-1] * 38 + "lkjihgfedcba")

# Small random cases over tiny and full alphabets.
for _ in range(30):
    alpha = r.choice(["ab", "abc", "abcd", AZ])
    a = random_string(r, r.randint(1, 12), alpha)
    b = random_string(r, r.randint(1, 12), alpha)
    emit(a, b)
for _ in range(5):
    a = random_string(r, r.randint(2, 15), "abcde")
    emit(a, subsequence_of(a))

# Medium random cases.
for _ in range(12):
    alpha = r.choice(["ab", "abcd", AZ])
    emit(random_string(r, r.randint(20, 400), alpha), random_string(r, r.randint(20, 400), alpha))

# Large cases at the limits.
for alpha in ["ab", "abcd", AZ, "xyz"]:
    emit(random_string(r, 1000, alpha), random_string(r, 1000, alpha))
a = random_string(r, 1000, "abc")
emit(a, a)
emit(a, a[::-1])
emit(random_string(r, 1000, "abcdef"), random_string(r, 1000, "defghi"))
