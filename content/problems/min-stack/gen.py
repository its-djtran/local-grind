import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(155)
LO, HI = -(2**31), 2**31 - 1
MAXCALLS = 3 * 10**4


def emit_seq(calls):
    """calls: list of ("push", val) / ("pop",) / ("top",) / ("getMin",). Validates guarantees."""
    assert len(calls) <= MAXCALLS
    ops, args, size = ["MinStack"], [[]], 0
    for c in calls:
        if c[0] == "push":
            assert LO <= c[1] <= HI
            size += 1
        else:
            assert size > 0, "pop/top/getMin on an empty stack"
            if c[0] == "pop":
                size -= 1
        ops.append(c[0])
        args.append(list(c[1:]))
    emit(ops, args)


def random_calls(m, lo, hi, p_push=0.45):
    out, size = [], 0
    for _ in range(m):
        if size == 0 or r.random() < p_push:
            out.append(("push", r.randint(lo, hi)))
            size += 1
        else:
            op = r.choice(["pop", "top", "getMin"])
            out.append((op,))
            size -= op == "pop"
    return out


Q = [("top",), ("getMin",)]
emit_seq([("push", 0), ("top",), ("getMin",)])
emit_seq([("push", LO), ("push", HI), ("getMin",), ("top",), ("pop",), ("getMin",), ("top",)])
emit_seq([("push", HI), ("push", LO), ("getMin",), ("pop",), ("getMin",)])
# Duplicate minimums: popping one copy must keep the other.
emit_seq([("push", 0), ("push", 1), ("push", 0), ("getMin",), ("pop",), ("getMin",), ("pop",), ("getMin",)])
emit_seq([("push", 2), ("push", 2), ("push", 2), ("pop",), ("getMin",), ("pop",), ("getMin",), ("top",)])
# Empty the stack completely, then reuse it.
emit_seq([("push", 5), ("pop",), ("push", 7), ("getMin",), ("top",), ("pop",), ("push", -7), ("getMin",)])
# Decreasing then popping back up; increasing pushes keep the minimum at the bottom.
emit_seq([("push", v) for v in range(10, 0, -1)] + [c for _ in range(10) for c in Q + [("pop",)]])
emit_seq([("push", v) for v in range(10)] + [c for _ in range(10) for c in Q + [("pop",)]])

for _ in range(30):
    emit_seq(random_calls(r.randint(3, 40), -5, 5))
for _ in range(15):
    emit_seq(random_calls(r.randint(40, 3000), LO, HI))

# Large cases: a deep stack queried thousands of times makes an O(n) getMin too slow.
half = MAXCALLS // 2
emit_seq([("push", r.randint(LO, HI)) for _ in range(half)] + [("getMin",)] * half)
emit_seq([("push", v) for v in range(half)] + [Q[i % 2] for i in range(half)])
emit_seq([("push", -v) for v in range(half)] + [("getMin",), ("pop",)] * (half // 2))
deep = [("push", r.randint(LO, HI)) for _ in range(10**4)]
emit_seq(deep + random_calls(MAXCALLS - 10**4, LO, HI, 0.5))
