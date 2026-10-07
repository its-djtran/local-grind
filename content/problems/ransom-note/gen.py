import sys
from collections import Counter

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(383)
MAX = 10**5
ABC = "abcdefghijklmnopqrstuvwxyz"


def brute(note, mag):
    remaining = list(mag)
    for ch in note:
        if ch not in remaining:
            return False
        remaining.remove(ch)
    return True


def check_and_emit(note, mag):
    assert 1 <= len(note) <= MAX and 1 <= len(mag) <= MAX
    assert set(note) <= set(ABC) and set(mag) <= set(ABC)
    if len(note) * len(mag) <= 10**6:
        assert brute(note, mag) == (not Counter(note) - Counter(mag))
    emit(note, mag)


def possible(n, m, alphabet):
    """A note of length n drawn out of a random magazine of length m >= n."""
    mag = random_string(r, m, alphabet)
    note = list(mag)
    r.shuffle(note)
    return "".join(note[:n]), mag


def impossible(n, m, alphabet):
    """Like possible(), but the note needs one more copy of some letter than exists."""
    while True:
        note, mag = possible(n - 1, m, alphabet)
        ch = r.choice(mag)
        note = list(note) + [ch] * (mag.count(ch) - note.count(ch) + 1)
        r.shuffle(note)
        if len(note) <= MAX:
            return "".join(note), mag


fixed = [
    ("a", "a"), ("a", "b"), ("ab", "ba"), ("abc", "ab"), ("aab", "baa"), ("aab", "bab"),
    ("z", "abcdefghijklmnopqrstuvwxyz"), ("zz", "abcdefghijklmnopqrstuvwxyz"),
    ("aaaa", "aaa"), ("bg", "efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj"),
    ("fffbfg", "effjfggbffjdgbjjhhdegh"), ("abcdef", "fedcba"),
]
for note, mag in fixed:
    check_and_emit(note, mag)
for _ in range(20):
    n = r.randint(1, 15)
    check_and_emit(random_string(r, n, "abc"), random_string(r, r.randint(1, 20), "abc"))
for _ in range(12):
    n = r.randint(1, 200)
    check_and_emit(*possible(n, r.randint(n, 400), r.choice([ABC, "abcde"])))
for _ in range(12):
    n = r.randint(2, 200)
    check_and_emit(*impossible(n, r.randint(n, 400), r.choice([ABC, "abcde"])))
# Large cases: valid notes force a full scan; the note-longer-than-magazine shortcut never helps.
for _ in range(4):
    check_and_emit(*possible(MAX, MAX, ABC))
for _ in range(2):
    check_and_emit(*possible(MAX // 2, MAX, ABC))
check_and_emit(*impossible(MAX, MAX, ABC))
check_and_emit("a" * MAX, "a" * MAX)
check_and_emit("a" * (MAX - 1) + "b", "a" * MAX)
check_and_emit(random_string(r, MAX), random_string(r, MAX // 2))
