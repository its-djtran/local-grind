import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(20)
OPEN, CLOSE = "([{", ")]}"
N = 10**4


def balanced(n_pairs):
    """A random balanced string with n_pairs bracket pairs."""
    out, stack = [], []
    remaining = n_pairs
    while remaining or stack:
        if remaining and (not stack or r.random() < 0.5):
            k = r.randrange(3)
            out.append(OPEN[k])
            stack.append(k)
            remaining -= 1
        else:
            out.append(CLOSE[stack.pop()])
    return "".join(out)


def nested(n_pairs):
    ks = [r.randrange(3) for _ in range(n_pairs)]
    return "".join(OPEN[k] for k in ks) + "".join(CLOSE[k] for k in reversed(ks))


def is_valid(s):
    pairs = {")": "(", "]": "[", "}": "{"}
    stack = []
    for ch in s:
        if ch in pairs:
            if not stack or stack.pop() != pairs[ch]:
                return False
        else:
            stack.append(ch)
    return not stack


def corrupt(s):
    """Breaks a balanced string in one of several ways (result is checked to be invalid)."""
    while True:
        chars = list(s)
        kind = r.randrange(4)
        i = r.randrange(len(chars))
        if kind == 0:  # change one bracket's kind
            c = chars[i]
            group = OPEN if c in OPEN else CLOSE
            chars[i] = r.choice([g for g in group if g != c])
        elif kind == 1:  # delete one character
            del chars[i]
        elif kind == 2:  # swap two adjacent characters
            if i + 1 < len(chars):
                chars[i], chars[i + 1] = chars[i + 1], chars[i]
        else:  # insert a stray bracket
            chars.insert(i, r.choice(OPEN + CLOSE))
        t = "".join(chars)
        if t and len(t) <= N and not is_valid(t):
            return t


for s in ["(", ")", "]", "{", "()", "[]", "{}", ")(", "][", "(((", ")))", "(])", "([]",
          "{[]}", "{[}]", "([{}])", "(){}}{", "((", "(()", "())(", "[({})](]", "{{{{}}}}"]:
    emit(s)
for n in range(1, 13):
    s = balanced(n)
    emit(s)
    emit(corrupt(s))
for _ in range(25):
    s = balanced(r.randint(5, 500))
    emit(s if r.random() < 0.5 else corrupt(s))
for _ in range(5):
    emit("".join(r.choice(OPEN + CLOSE) for _ in range(r.randint(2, 40))))
# Large cases: deep nesting and long flat runs, valid and broken at the very end.
deep = nested(N // 2)
emit(deep)
emit(deep[:-1] + ("]" if deep[-1] != "]" else ")"))
emit("(" * N)
emit("()" * (N // 2))
emit("([{" * (N // 6) + "}])" * (N // 6) + "(}")
emit(balanced(N // 2))
emit(corrupt(balanced(N // 2)))
emit(nested(N // 4) + nested(N // 4))
