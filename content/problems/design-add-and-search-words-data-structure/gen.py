import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(211)
LOWER = "abcdefghijklmnopqrstuvwxyz"


class Seq:
    def __init__(self):
        self.ops, self.args, self.words = ["WordDictionary"], [[]], []

    def add(self, word):
        assert 1 <= len(word) <= 25 and word.isalpha() and word.islower()
        self.ops.append("addWord")
        self.args.append([word])
        self.words.append(word)

    def search(self, pattern):
        assert 1 <= len(pattern) <= 25 and pattern.count(".") <= 2
        assert all(c == "." or c in LOWER for c in pattern)
        self.ops.append("search")
        self.args.append([pattern])

    def emit(self):
        assert len(self.ops) - 1 <= 10**4
        emit(self.ops, self.args)


def dotted(s, max_dots=2):
    s = list(s)
    for i in r.sample(range(len(s)), min(len(s), r.randint(0, max_dots))):
        s[i] = "."
    return "".join(s)


def query(seq, alphabet, lo, hi):
    roll = r.random()
    if seq.words and roll < 0.45:
        return dotted(r.choice(seq.words))  # usually present
    if seq.words and roll < 0.6:
        w = r.choice(seq.words)
        if len(w) > 1:
            return dotted(w[: r.randint(1, len(w) - 1)])  # a strict prefix
        return dotted(w + r.choice(alphabet))
    if seq.words and roll < 0.7:
        w = r.choice(seq.words)
        return dotted((w + random_string(r, r.randint(1, 2), alphabet))[:25])
    return dotted(random_string(r, r.randint(lo, hi), alphabet))


def random_seq(n_ops, alphabet, lo, hi, add_frac=0.45):
    seq = Seq()
    for _ in range(n_ops):
        if r.random() < add_frac:
            seq.add(random_string(r, r.randint(lo, hi), alphabet))
        else:
            seq.search(query(seq, alphabet, lo, hi))
    return seq


def fixed(steps):
    seq = Seq()
    for op, s in steps:
        (seq.add if op == "add" else seq.search)(s)
    seq.emit()


# Hand-written edge cases.
fixed([("search", "a"), ("search", "."), ("add", "a"), ("search", "a"), ("search", "."),
       ("search", ".."), ("search", "b")])
fixed([("add", "a"), ("add", "ab"), ("search", "a."), ("search", ".b"), ("search", "."),
       ("search", ".b."), ("search", "b.")])
fixed([("add", "abc"), ("search", "ab"), ("search", "abcd"), ("search", "a.c"), ("search", "..c"),
       ("search", ".b."), ("search", "a.d")])
fixed([("add", "at"), ("add", "and"), ("add", "an"), ("add", "add"), ("search", "a"), ("search", ".at"),
       ("add", "bat"), ("search", ".at"), ("search", "an."), ("search", "a.d."), ("search", "b."),
       ("search", "a.d"), ("search", ".")])
fixed([("add", "z" * 25), ("search", "z" * 24), ("search", "z" * 23 + ".."), ("search", "." + "z" * 24),
       ("search", "z" * 12 + "y" + "z" * 12), ("search", "z" * 12 + "." + "z" * 12)])
fixed([("add", "bad"), ("add", "bad"), ("search", "bad"), ("search", "b.d"), ("search", "..d")])
fixed([("add", w) for w in ["ab", "ac", "bc", "bd"]] + [("search", s) for s in [".d", "a.", "c.", "..", ".a"]])

# Small random sequences; tiny alphabets make dots match many paths.
for _ in range(30):
    alphabet = r.choice(["ab", "abc", "xyz", LOWER[:5], LOWER])
    random_seq(r.randint(1, 40), alphabet, 1, r.randint(1, 6)).emit()
for _ in range(10):
    random_seq(r.randint(100, 1500), r.choice(["ab", "abcd", LOWER[:8], LOWER]), 1, 25).emit()

# Large cases: thousands of equal-length words, then thousands of wildcard queries that mostly
# fail, so comparing each query with every stored word is far too slow.
for length, alphabet in [(10, LOWER), (8, LOWER[:12])]:
    seq = Seq()
    for _ in range(4000):
        seq.add(random_string(r, length, alphabet))
    for _ in range(6000):
        if r.random() < 0.2:
            seq.search(dotted(r.choice(seq.words), 2))
        else:
            seq.search(dotted(random_string(r, length, alphabet), r.randint(1, 2)))
    seq.emit()
# Dense prefixes plus leading dots: the trie must branch widely right at the start.
seq = Seq()
for _ in range(5000):
    seq.add(random_string(r, r.randint(3, 25), LOWER[:6]))
for _ in range(5000):
    seq.search(".." + random_string(r, r.randint(1, 23), LOWER[:6]))
seq.emit()
seq = random_seq(10**4, LOWER, 1, 25, 0.5)
seq.emit()
