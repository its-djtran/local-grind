import sys
from collections import Counter

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(409)
MAX = 2000
LOWER = "abcdefghijklmnopqrstuvwxyz"
BOTH = LOWER + LOWER.upper()


def answer(s):
    counts = Counter(s)
    return sum(c // 2 * 2 for c in counts.values()) + (1 if any(c % 2 for c in counts.values()) else 0)


def check_and_emit(s):
    assert 1 <= len(s) <= MAX and set(s) <= set(BOTH)
    odd = {ch for ch, c in Counter(s).items() if c % 2}
    assert answer(s) == len(s) - len(odd) + (1 if odd else 0)
    emit(s)


fixed = ["a", "A", "bb", "ab", "Aa", "aA", "ccc", "aaab", "abccccdd", "AAAAaaaa", "abcABC",
         "civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicated", "zzzzZ", "bananas",
         "aabbccdd", "abcdefghijklmnopqrstuvwxyz", BOTH]
for s in fixed:
    check_and_emit(s)
for _ in range(25):
    check_and_emit(random_string(r, r.randint(1, 12), r.choice(["ab", "aA", "abc", BOTH])))
for _ in range(20):
    check_and_emit(random_string(r, r.randint(12, 500), r.choice([LOWER, BOTH, "xyzXYZ"])))
# Maximal strings: all pairs, all odd counts, one letter, every letter distinct-ish.
check_and_emit(random_string(r, MAX, BOTH))
check_and_emit(random_string(r, MAX - 1, BOTH))
check_and_emit("a" * MAX)
check_and_emit("Z" * (MAX - 1))
half = list(random_string(r, MAX // 2, BOTH))
check_and_emit("".join(r.sample(half * 2, MAX)))
check_and_emit((BOTH * 40)[:MAX - 52] + BOTH)  # 52 letters, many with odd counts
check_and_emit("".join(ch * 37 for ch in BOTH)[:MAX])
for s in ("abccccdd", "a", "Aa", "bb"):
    assert answer(s) == {"abccccdd": 7, "a": 1, "Aa": 1, "bb": 2}[s]
