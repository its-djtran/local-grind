import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(139)


def dictionary(count, alpha, lo, hi):
    count = min(count, sum(len(alpha) ** k for k in range(lo, hi + 1)) // 2)
    words = set()
    while len(words) < count:
        words.add(random_string(r, r.randint(lo, hi), alpha))
    out = sorted(words)
    r.shuffle(out)
    return out


def built(words, n):
    """A string of length <= n made by concatenating dictionary words, sometimes corrupted."""
    s = ""
    while True:
        w = r.choice(words)
        if len(s) + len(w) > n:
            break
        s += w
    if not s:
        s = min(words, key=len)
    if r.random() < 0.5:
        i = r.randrange(len(s))
        s = s[:i] + r.choice("abcdefghijklmnopqrstuvwxyz") + s[i + 1:]
    return s[:n]


A_RUNS = ["a" * k for k in range(1, 21)]

# Edge cases.
emit("a", ["a"])
emit("a", ["b"])
emit("ab", ["a", "b"])
emit("ab", ["ab", "a"])
emit("cars", ["car", "ca", "rs"])                   # greedy longest-first fails
emit("aaaaaaa", ["aaaa", "aaa"])
emit("goalspecial", ["go", "goal", "goals", "special"])
emit("bb", ["a", "b", "bbb", "bbbb"])
emit("abcd", ["a", "abc", "b", "cd"])
emit("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", ["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa"])
emit("x" * 300, ["x" * 20])
emit("x" * 300, ["x" * 7])
emit("z" * 300, ["z" * 13, "z" * 17])
emit("q" * 20, ["q" * 20])
emit("abcdefghijklmnopqrst" * 15, ["abcdefghijklmnopqrst"])

# Small random cases over tiny alphabets (many overlapping words).
for _ in range(30):
    alpha = r.choice(["ab", "abc"])
    words = dictionary(r.randint(1, 6), alpha, 1, 4)
    emit(built(words, r.randint(1, 20)), words)

# Medium random cases.
for _ in range(12):
    alpha = r.choice(["ab", "abcd", "abcdefghijklmnopqrstuvwxyz"])
    words = dictionary(r.randint(5, 200), alpha, 1, r.choice([5, 20]))
    emit(built(words, r.randint(20, 300)), words)

# Large cases: backtracking blows up on these, especially the false ones.
emit("a" * 299 + "b", A_RUNS)
emit("b" + "a" * 299, A_RUNS)
emit("a" * 150 + "b" + "a" * 149, A_RUNS)
emit("a" * 300, A_RUNS[1:])                         # true: even lengths are enough
emit("ab" * 149 + "aa", ["ab", "a", "b", "abab", "ba", "aba", "bab"])
emit("ab" * 149 + "c" + "a", ["ab", "a", "b", "abab", "ba", "aba", "bab"])
# 1000 words: every run of a's plus words that can never line up with the final "b".
noise = [w for w in dictionary(1200, "ab", 2, 20) if "b" in w and w.rstrip("b") != w.rstrip("b").replace("b", "")]
noise = [w for w in noise if not (w.endswith("b") and set(w[:-1]) <= {"a"})]
emit("a" * 299 + "b", A_RUNS + noise[:980])
for _ in range(4):
    words = dictionary(1000, r.choice(["ab", "abc", "abcdefghijklmnopqrstuvwxyz"]), 1, 20)
    emit(built(words, 300), words)
