import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(150)
LO, HI = -(2**31), 2**31 - 1
OPS = "+-*/"


def trunc_div(a, b):
    q = abs(a) // abs(b)
    return q if (a < 0) == (b < 0) else -q


def apply(a, b, op):
    if op == "+":
        return a + b
    if op == "-":
        return a - b
    if op == "*":
        return a * b
    return trunc_div(a, b)


def valid(a, b, op):
    return not (op == "/" and b == 0) and LO <= apply(a, b, op) <= HI


def evaluate(tokens):
    """Independent checker: verifies every guarantee and returns the value."""
    stack = []
    for t in tokens:
        if t in OPS:
            assert len(stack) >= 2
            b, a = stack.pop(), stack.pop()
            assert valid(a, b, t)
            stack.append(apply(a, b, t))
        else:
            assert -200 <= int(t) <= 200 and str(int(t)) == t
            stack.append(int(t))
    assert len(stack) == 1
    return stack[0]


def expression(numbers, p_op, weights=(3, 3, 2, 2), lo=-200, hi=200):
    """Random valid RPN with `numbers` operands. p_op: chance to reduce when possible
    (0 puts every operator at the end, 1 gives a left-deep chain)."""
    tokens, stack, pushed = [], [], 0
    while pushed < numbers or len(stack) > 1:
        if pushed < numbers and (len(stack) < 2 or r.random() >= p_op):
            v = r.randint(lo, hi)
            tokens.append(str(v))
            stack.append(v)
            pushed += 1
            continue
        b, a = stack.pop(), stack.pop()
        op = r.choices(OPS, weights)[0]
        if not valid(a, b, op):
            op = next(o for o in "+-" if valid(a, b, o))
        tokens.append(op)
        stack.append(apply(a, b, op))
    assert len(tokens) == 2 * numbers - 1 <= 10**4
    evaluate(tokens)
    return tokens


def emit_tokens(tokens):
    evaluate(tokens)
    emit(tokens)


for t in (["5"], ["-200"], ["200"], ["0"]):
    emit_tokens(t)
emit_tokens(["3", "4", "-"])
emit_tokens(["4", "3", "-"])
# Truncation toward zero, including negative quotients where floor division differs.
emit_tokens(["7", "-2", "/"])
emit_tokens(["-7", "2", "/"])
emit_tokens(["-7", "-2", "/"])
emit_tokens(["7", "2", "/"])
emit_tokens(["6", "-3", "/"])
emit_tokens(["1", "-200", "/"])
emit_tokens(["0", "-5", "/"])
# Intermediate values near the 32-bit limits: 200^4 = 1.6 * 10^9.
emit_tokens(["200", "200", "*", "200", "*", "200", "*"])
emit_tokens(["200", "200", "*", "200", "*", "200", "*", "-1", "*"])
emit_tokens(["-200", "200", "*", "200", "*", "200", "*", "-3", "/"])
# 200^4 + 68 * 200^3 = 2144000000 and its negation, just inside the 32-bit range.
emit_tokens(["200", "200", "*", "200", "*", "200", "*", "200", "200", "*", "200", "*", "68", "*", "+"])
emit_tokens(["-200", "200", "*", "200", "*", "200", "*", "200", "200", "*", "200", "*", "68", "*", "-"])
emit_tokens(["2", "3", "4", "*", "+"])
emit_tokens(["1", "2", "+", "3", "+", "4", "+", "5", "+"])
emit_tokens(["1", "2", "3", "4", "5", "-", "-", "-", "-"])

for _ in range(25):
    emit(expression(r.randint(2, 8), r.random(), lo=-10, hi=10))
for _ in range(15):
    emit(expression(r.randint(8, 300), r.random()))
# Division-heavy expressions so truncation errors surface.
for _ in range(6):
    emit(expression(r.randint(5, 60), r.random(), weights=(1, 1, 2, 5)))

# Large cases at the maximum length. With every operator at the end, rescanning for the first
# operator is quadratic.
for _ in range(5):
    emit(expression(5000, 0.0))
emit(expression(5000, 0.0, weights=(1, 1, 0, 0)))
emit(expression(5000, 1.0))
emit(expression(5000, 0.5))
