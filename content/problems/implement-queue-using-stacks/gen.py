import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(232)
MAXCALLS = 100


def emit_seq(calls):
    """calls: list of ("push", x) / ("pop",) / ("peek",) / ("empty",). Validates guarantees."""
    assert len(calls) <= MAXCALLS
    ops, args, size = ["MyQueue"], [[]], 0
    for c in calls:
        if c[0] == "push":
            assert 1 <= c[1] <= 9
            size += 1
        elif c[0] in ("pop", "peek"):
            assert size > 0, "pop/peek on an empty queue"
            size -= c[0] == "pop"
        ops.append(c[0])
        args.append(list(c[1:]))
    emit(ops, args)


def random_calls(m, p_push=0.45):
    out, size = [], 0
    for _ in range(m):
        if size == 0 and r.random() < 0.2:
            out.append(("empty",))
        elif size == 0 or r.random() < p_push:
            out.append(("push", r.randint(1, 9)))
            size += 1
        else:
            op = r.choice(["pop", "pop", "peek", "empty"])
            out.append((op,))
            size -= op == "pop"
    return out


P = lambda x: ("push", x)
emit_seq([("empty",)])
emit_seq([P(1), ("empty",), ("pop",), ("empty",)])
emit_seq([P(1), P(2), P(3), ("pop",), ("pop",), ("pop",), ("empty",)])
emit_seq([P(9), ("peek",), ("peek",), ("pop",), ("empty",)])
# Pushes interleaved with pops: refilling the outbox while it is non-empty breaks FIFO order.
emit_seq([P(1), P(2), ("pop",), P(3), P(4), ("pop",), ("peek",), ("pop",), P(5), ("pop",), ("pop",), ("empty",)])
emit_seq([P(1), P(2), ("peek",), P(3), ("pop",), ("pop",), P(4), ("peek",), ("pop",), ("pop",), ("empty",)])
emit_seq([P(i % 9 + 1) for i in range(50)] + [("pop",)] * 50)
emit_seq([P(i % 9 + 1) for i in range(49)] + [("pop",)] * 49 + [("empty",)])
emit_seq([c for i in range(33) for c in (P(i % 9 + 1), ("peek",), ("pop",))])
emit_seq([P(5)] * 10 + [("pop",), ("empty",)] * 10)
for _ in range(30):
    emit_seq(random_calls(r.randint(2, 20)))
for _ in range(20):
    emit_seq(random_calls(r.randint(20, MAXCALLS), r.choice([0.4, 0.5, 0.6])))
