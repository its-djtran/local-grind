import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(269)
ALPHA = "abcdefghijklmnopqrstuvwxyz"


def sorted_words(n, max_len, letters):
    """n random words over `letters`, sorted by a random hidden alphabet (always valid)."""
    alphabet = list(letters)
    r.shuffle(alphabet)
    rank = {c: i for i, c in enumerate(alphabet)}
    words = []
    for _ in range(n):
        if words and r.random() < 0.3:
            base = r.choice(words)  # share a prefix with an existing word to create rules
            cut = r.randint(0, len(base))
            w = base[:cut] + "".join(r.choice(letters) for _ in range(r.randint(0 if cut else 1, 3)))
            w = w[:max_len] or r.choice(letters)
        else:
            w = "".join(r.choice(letters) for _ in range(r.randint(1, max_len)))
        words.append(w)
    words.sort(key=lambda w: [rank[c] for c in w])
    return words


def break_order(words):
    """Make the list very likely unsortable by swapping two non-equal words."""
    w = list(words)
    for _ in range(20):
        i, j = sorted(r.sample(range(len(w)), 2))
        if w[i] != w[j]:
            w[i], w[j] = w[j], w[i]
            break
    return w


emit(["a"])
emit(["abc"])
emit(["z", "z"])
emit(["ab", "adc"])
emit(["abc", "ab"])
emit(["ab", "abc"])
emit(["a", "b", "a"])
emit(["ac", "ab", "zc", "zb"])
emit(["wrt", "wrtkj"])
emit(["wrtkj", "wrt"])
emit(["z", "x", "a", "zb", "zx"])
emit(["aba"])
emit(["zy", "zx"])
emit(["a", "b", "c", "a"])
emit(["bc", "b", "cbc"])
emit(list(ALPHA))
emit(list(reversed(ALPHA)))
emit([ALPHA * 3 + "abcdefghijklmnopqrstuv"])
emit(["x" * 100] * 100)

for _ in range(20):
    k = r.randint(2, 6)
    letters = r.sample(ALPHA, k)
    words = sorted_words(r.randint(2, 8), 4, letters)
    emit(words if r.random() < 0.6 else break_order(words))
for _ in range(15):
    letters = r.sample(ALPHA, r.randint(5, 26))
    words = sorted_words(r.randint(10, 100), r.randint(3, 20), letters)
    emit(words if r.random() < 0.6 else break_order(words))
for _ in range(5):
    words = sorted_words(100, 100, ALPHA)
    emit(words)
# A full chain of 26 rules defines a unique alphabet.
alphabet = list(ALPHA)
r.shuffle(alphabet)
emit(alphabet[:])
emit(["q" + c for c in alphabet] + ["r"])
# A long cycle: a < b < ... < z < a.
emit([ALPHA[i] + ALPHA[(i + 1) % 26] for i in range(26)] + ["za"])
emit(["x" + ALPHA[i] for i in range(26)] + ["x" + "a"])
