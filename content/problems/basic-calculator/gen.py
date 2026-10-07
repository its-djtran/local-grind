import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(224)
N = 3 * 10**5
LO, HI = -(2**31), 2**31 - 1


def check(s):
    """Validates the grammar and the 32-bit guarantee; returns the value of the expression."""
    assert 1 <= len(s) <= N and set(s) <= set("0123456789+-() ")
    toks, i = [], 0
    while i < len(s):
        c = s[i]
        if c.isdigit():
            j = i
            while j < len(s) and s[j].isdigit():
                j += 1
            toks.append(int(s[i:j]))
            i = j
            continue
        if c != " ":
            toks.append(c)
        i += 1
    # Each frame: [total, pending sign, expecting operand?]
    frames = [[0, 1]]
    expect = True  # an operand (number, '(' or unary '-') must come next
    prev = None
    for t in toks:
        top = frames[-1]
        if isinstance(t, int):
            assert expect and t <= HI
            top[0] += top[1] * t
            assert LO <= top[0] <= HI
            expect = False
        elif t in "+-":
            if expect:  # unary minus only at the start or right after '('
                assert t == "-" and prev in (None, "(")
            top[1] = 1 if t == "+" else -1
            expect = True
        elif t == "(":
            assert expect
            frames.append([0, 1])
        else:  # ')'
            assert not expect and len(frames) > 1
            inner = frames.pop()[0]
            top = frames[-1]
            top[0] += top[1] * inner
            assert LO <= top[0] <= HI
        prev = t
    assert not expect and len(frames) == 1
    return frames[0][0]


def put(s):
    value = check(s)
    if s.count("(") < 50 and len(s) <= 5000:
        assert eval(s) == value, s
    emit(s)


def gap(p):
    return " " * (r.random() < p) * r.choice([1, 1, 1, 2, 3])


def random_expr(target, max_num=100, p_open=0.25, p_close=0.25, p_unary=0.2, p_space=0.3):
    """Random walk over the grammar, closing every group once the length budget runs out."""
    out, depth, length = [], 0, 0
    margin = 30 if target > 200 else 3
    after_open = True  # at the very start, a unary minus is allowed too

    def add(tok):
        nonlocal length
        tok = gap(p_space) + tok
        out.append(tok)
        length += len(tok)

    while True:
        # Operand position.
        if after_open and r.random() < p_unary:
            add("-")
        if length + depth + margin < target and r.random() < p_open:
            add("(")
            depth += 1
            after_open = True
            continue
        add(str(r.randint(0, max_num)))
        after_open = False
        # Operator position.
        while depth and r.random() < p_close:
            add(")")
            depth -= 1
        if length + depth + margin >= target:
            break
        add(r.choice("+-"))
    out.append(")" * depth + gap(p_space))
    return "".join(out)


def chain(n_levels, op="-"):
    """1-(2-(3-(...))) nested n_levels deep."""
    head = "".join("%d%s(" % (r.randint(1, 9), op) for _ in range(n_levels))
    return head + str(r.randint(1, 9)) + ")" * n_levels


for s in ["0", "1", "-1", "2147483647", "-2147483647", "-(2147483647)", "1 + 1", " 2-1 + 2 ",
          "1-1", "1 - (-2)", "-(2 + 3)", "-(-(-5))", "(7)", "((((7))))", "(1+(4+5+2)-3)+(6+8)",
          "2-(5-6)", "- (3 + (4 + 5))", "1-(     -2)", "   30", "3 ", "(12)-(-(3))-4",
          "10 - (2 + 3) - (4 - (5 - 6))", "(0)-0", "2147483647 - 2147483647 + 5",
          "1-2147483647-1", "-(1)+(-1)", "(-1)-(-(-(1)))", "(((1)-((2))))"]:
    put(s)
for _ in range(15):
    put(random_expr(r.randint(5, 40), max_num=20))
for _ in range(20):
    put(random_expr(r.randint(40, 2000), max_num=r.choice([9, 1000, 10**6])))
for _ in range(4):
    put(chain(r.randint(50, 500)))
# Large cases at the length limit.
put(chain((N - 1) // 4))                      # ~75000 nested levels: 1-(2-(3-(...)))
put("-(" * ((N - 1) // 3) + "7" + ")" * ((N - 1) // 3))
put("(" * (N // 2 - 1) + "42" + ")" * (N // 2 - 1))
k = (N - 2) // 4
put("(" * k + "1" + "".join("%s%d)" % (r.choice("+-"), r.randint(0, 9)) for _ in range(k)))
put("+".join("1" for _ in range(N // 2)))
put("".join("%s%d" % ("-" if i % 2 else "+", r.randint(0, 9999)) for i in range(N // 6))[1:])
put(random_expr(N, max_num=10**4, p_space=0.5))
put(random_expr(N, max_num=9, p_open=0.5, p_close=0.1, p_space=0.0))
put(random_expr(N, max_num=100, p_open=0.4, p_close=0.4, p_unary=0.5, p_space=0.2))
