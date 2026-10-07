import string
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(3)
PRINTABLE = "".join(chr(c) for c in range(32, 127))  # letters, digits, symbols and space
N = 5 * 10**4

emit("")
emit(" ")
emit("a")
emit("au")
emit("aa")
emit("dvdf")
emit("abba")
emit("tmmzuxt")
emit("  ")
emit("a b c")
emit("abcdefghijklmnopqrstuvwxyz")
emit(PRINTABLE)
emit(PRINTABLE + PRINTABLE)
emit("abcdeafghij")
emit('"\\"\\')
for n in range(2, 16):
    emit(random_string(r, n, "abc"))
for _ in range(18):
    emit(random_string(r, r.randint(10, 200), r.choice(["ab", "abcde", string.ascii_letters, PRINTABLE])))
for _ in range(10):
    emit(random_string(r, r.randint(1000, 5000), r.choice([string.ascii_lowercase, PRINTABLE])))
emit("a" * N)
emit(random_string(r, N, PRINTABLE))
# Long runs of all-distinct characters make "extend from each start" scan ~95 chars per start.
for _ in range(10):
    chars = list(PRINTABLE)
    out = []
    while len(out) < N:
        r.shuffle(chars)
        out += chars
    emit("".join(out[:N]))
cycle = PRINTABLE * (N // len(PRINTABLE) + 1)
emit(cycle[:N])
