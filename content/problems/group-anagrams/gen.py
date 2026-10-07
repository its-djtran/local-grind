import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(49)


def shuffled(s):
    chars = list(s)
    r.shuffle(chars)
    return "".join(chars)


def mixed(n, groups, lo, hi, alphabet="abcdefghijklmnopqrstuvwxyz"):
    """n words drawn from `groups` random base words, each one a shuffled copy of its base."""
    bases = [random_string(r, r.randint(lo, hi), alphabet) for _ in range(groups)]
    return [shuffled(r.choice(bases)) for _ in range(n)]


emit(["", ""])
emit(["", "b"])
emit(["a", "b"])
emit(["ab", "ba", "ab"])
emit(["abc", "bca", "cab", "abd"])
emit(["aab", "abb", "bab", "baa"])
emit(["z"] * 5)
emit(["", "a", "", "a", "aa"])
emit(["ac", "c"])
emit(["ddddddddddg", "dgggggggggg"])
emit(["tea", "and", "ace", "ad", "eat", "dans"])
for _ in range(25):
    emit(mixed(r.randint(1, 12), r.randint(1, 5), 0, 4, r.choice(["ab", "abc", "abcdef"])))
for _ in range(15):
    emit(mixed(r.randint(10, 300), r.randint(1, 60), 0, 12))
emit([""] * 10**4)
emit(["a"] * 10**4)
emit(["abcdefghijklmnopqrstuvwxyz" * 3 + "abcdefghijklmnopqrstuv"] * 50)
emit(mixed(10**4, 5, 1, 10))
emit(mixed(2000, 2000, 100, 100))
# Many distinct groups: comparing each word against every existing group is quadratic.
for _ in range(5):
    emit(mixed(10**4, r.randint(5000, 9000), 4, 10))
emit([random_string(r, 6, "abcdefgh") for _ in range(10**4)])
