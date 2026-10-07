import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(208)
LOWER = "abcdefghijklmnopqrstuvwxyz"


class Seq:
    def __init__(self):
        self.ops, self.args, self.words = ["Trie"], [[]], []

    def add(self, op, s):
        assert 1 <= len(s) <= 2000 and s.isalpha() and s.islower()
        self.ops.append(op)
        self.args.append([s])
        if op == "insert":
            self.words.append(s)

    def emit(self):
        assert len(self.ops) - 1 <= 3 * 10**4
        emit(self.ops, self.args)


def query_arg(seq, alphabet, lo, hi):
    """A query string: an inserted word, a prefix or extension of one, or random."""
    roll = r.random()
    if seq.words and roll < 0.3:
        return r.choice(seq.words)
    if seq.words and roll < 0.6:
        w = r.choice(seq.words)
        return w[: r.randint(1, len(w))]
    if seq.words and roll < 0.75:
        w = r.choice(seq.words)
        return (w + random_string(r, r.randint(1, 3), alphabet))[:2000]
    return random_string(r, r.randint(lo, hi), alphabet)


def random_seq(n_ops, alphabet, lo, hi, insert_frac=0.4):
    seq = Seq()
    for _ in range(n_ops):
        if r.random() < insert_frac or not seq.words:
            if seq.words and r.random() < 0.2:
                w = r.choice(seq.words)
                word = w[: r.randint(1, len(w))]  # a prefix of an existing word
            else:
                word = random_string(r, r.randint(lo, hi), alphabet)
            seq.add("insert", word)
        else:
            seq.add(r.choice(("search", "startsWith")), query_arg(seq, alphabet, lo, hi))
    return seq


def fixed(ops_args):
    seq = Seq()
    for op, s in ops_args:
        seq.add(op, s)
    seq.emit()


# Hand-written edge cases.
fixed([("insert", "a"), ("search", "a"), ("startsWith", "a"), ("search", "b"), ("startsWith", "b")])
fixed([("search", "a"), ("startsWith", "a"), ("insert", "a"), ("search", "a")])
fixed([("insert", "apple"), ("search", "app"), ("startsWith", "apple"), ("startsWith", "apples"),
       ("search", "apples"), ("insert", "apples"), ("search", "apple"), ("search", "apples")])
fixed([("insert", "abc"), ("insert", "abc"), ("search", "abc"), ("search", "ab"), ("startsWith", "abcd")])
fixed([("insert", "ab"), ("insert", "abcd"), ("search", "abc"), ("startsWith", "abc"), ("search", "abcd"),
       ("search", "ab"), ("search", "a")])
fixed([("insert", "z" * 2000), ("search", "z" * 1999), ("startsWith", "z" * 1999), ("search", "z" * 2000),
       ("startsWith", "z" * 2000), ("startsWith", "y")])
fixed([("insert", "hello"), ("insert", "help"), ("insert", "hell"), ("startsWith", "hel"),
       ("search", "hel"), ("search", "hell"), ("startsWith", "helpx"), ("search", "help")])
fixed([("startsWith", "q"), ("search", "q")] + [("insert", c) for c in LOWER] + [("search", c) for c in LOWER[::-1]])

# Small random sequences; tiny alphabets force many shared prefixes.
for _ in range(30):
    alphabet = r.choice(["ab", "abc", "xyz", LOWER[:6], LOWER])
    random_seq(r.randint(1, 40), alphabet, 1, r.randint(1, 8)).emit()
for _ in range(10):
    random_seq(r.randint(100, 1000), r.choice(["ab", "abcd", LOWER]), 1, 15).emit()

# Long words: shared prefixes of length up to 2000.
seq = Seq()
base = random_string(r, 2000, "ab")
for _ in range(60):
    k = r.randint(1000, 1999)
    seq.add("insert", base[:k] + random_string(r, 2000 - k, "ab"))
for _ in range(60):
    w = r.choice(seq.words)
    op = r.choice(("search", "startsWith"))
    seq.add(op, w if r.random() < 0.5 else w[: r.randint(1000, 2000)])
seq.emit()

# Large cases: many stored words, then many prefix queries that mostly miss, so scanning the
# stored words for each query is far too slow.
for alphabet, lo, hi in [(LOWER, 5, 12), (LOWER[:10], 6, 10)]:
    seq = Seq()
    for _ in range(12000):
        seq.add("insert", random_string(r, r.randint(lo, hi), alphabet))
    for _ in range(18000):
        roll = r.random()
        if roll < 0.6:
            seq.add("startsWith", random_string(r, r.randint(lo, hi), alphabet))
        elif roll < 0.75:
            w = r.choice(seq.words)
            seq.add("startsWith", w[: r.randint(1, len(w))])
        else:
            seq.add("search", query_arg(seq, alphabet, lo, hi))
    seq.emit()
random_seq(10**4, LOWER[:4], 3, 14, 0.5).emit()

# Many maximum-length words: storing every prefix of every word needs O(L^2) memory per word.
seq = Seq()
for _ in range(480):
    seq.add("insert", random_string(r, 2000))
for _ in range(40):
    w = r.choice(seq.words)
    seq.add(r.choice(("search", "startsWith")), w[: r.randint(1, 2000)])
seq.emit()
