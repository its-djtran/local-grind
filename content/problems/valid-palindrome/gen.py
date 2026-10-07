import string
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(125)
N = 2 * 10**5
PRINTABLE = [chr(c) for c in range(32, 127)]
ALNUM = string.ascii_letters + string.digits
NOISE = [c for c in PRINTABLE if not c.isalnum()]


def expected(s):
    cleaned = [ch.lower() for ch in s if ch.isalnum()]
    return cleaned == cleaned[::-1]


def palindrome_core(n, alphabet):
    half = [r.choice(alphabet) for _ in range(n // 2)]
    mid = [r.choice(alphabet)] if n % 2 else []
    return half + mid + half[::-1]


def dress(core, noise_prob):
    """Randomizes letter case and sprinkles non-alphanumeric characters between characters."""
    out = []
    for ch in core:
        while r.random() < noise_prob:
            out.append(r.choice(NOISE))
        out.append(ch.upper() if ch.isalpha() and r.random() < 0.5 else ch)
    while r.random() < noise_prob:
        out.append(r.choice(NOISE))
    return "".join(out)


def palindrome(n, alphabet, noise_prob):
    return dress(palindrome_core(n, alphabet), noise_prob)


def near_palindrome(n, alphabet, noise_prob):
    """A palindrome with one character changed, so it is (checked to be) not a palindrome."""
    while True:
        core = palindrome_core(n, alphabet)
        k = r.randrange(len(core))
        core[k] = r.choice(alphabet)
        s = dress(core, noise_prob)
        if not expected(s):
            return s


for s in [" ", "a", "A", ".", "0P", "P0P", "ab", "Aa", "a.", ".,", "1b1", "1a2", "aA1Aa",
          "Was it a car or a cat I saw?", "No 'x' in Nixon", "ab_a", "0", "a1b2b1a", "!!a!!b!!",
          "Zz", "9:9", "`l;`` 1o1 ??;l`", "1234321", "12344321", "123421"]:
    emit(s)
for n in range(1, 16):
    emit(palindrome(n, "abc01", 0.3))
    emit(near_palindrome(max(n, 2), "abc01", 0.3))
for _ in range(15):
    emit("".join(r.choice(PRINTABLE) for _ in range(r.randint(1, 30))))
for _ in range(10):
    n = r.randint(50, 3000)
    emit(palindrome(n, ALNUM, 0.2) if r.random() < 0.5 else near_palindrome(n, ALNUM, 0.2))
# Large cases at the length limit.
emit(palindrome(N // 2, ALNUM, 0.5)[:N])
emit(near_palindrome(N // 2, "ab", 0.5)[:N])
emit("".join(r.choice(NOISE) for _ in range(N)))
emit("a" * N)
core = palindrome_core(N, string.ascii_lowercase)
core[N // 2] = "a" if core[N // 2] != "a" else "b"
emit("".join(core))  # mismatch only in the middle
s = palindrome(N - 10, ALNUM, 0.0)
emit(s + "".join(r.choice(NOISE) for _ in range(10)))
