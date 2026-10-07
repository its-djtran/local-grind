import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(146)
MAXKEY = 10**4
MAXVAL = 10**5


def emit_seq(capacity, calls):
    """calls: list of ("get", key) or ("put", key, value)."""
    assert 1 <= capacity <= 3000 and len(calls) <= 2 * 10**5
    ops, args = ["LRUCache"], [[capacity]]
    for c in calls:
        assert 0 <= c[1] <= MAXKEY and (c[0] == "get" or 0 <= c[2] <= MAXVAL)
        ops.append(c[0])
        args.append(list(c[1:]))
    emit(ops, args)


def random_calls(m, key_hi, p_get=0.5, val_hi=MAXVAL):
    out = []
    for _ in range(m):
        k = r.randint(0, key_hi)
        out.append(("get", k) if r.random() < p_get else ("put", k, r.randint(0, val_hi)))
    return out


# Edge cases.
emit_seq(1, [("get", 0)])
emit_seq(1, [("put", 0, 0), ("get", 0)])
emit_seq(1, [("put", 1, 1), ("put", 2, 2), ("get", 1), ("get", 2)])
emit_seq(1, [("put", 2, 1), ("get", 2), ("put", 3, 2), ("get", 2), ("get", 3)])
# Updating an existing key must not evict anything, and must refresh it.
emit_seq(2, [("put", 1, 1), ("put", 2, 2), ("put", 1, 10), ("put", 3, 3), ("get", 1), ("get", 2), ("get", 3)])
emit_seq(2, [("put", 2, 1), ("put", 2, 2), ("get", 2), ("put", 1, 1), ("put", 4, 1), ("get", 2)])
# A get must refresh recency.
emit_seq(2, [("put", 1, 1), ("put", 2, 2), ("get", 1), ("put", 3, 3), ("get", 2), ("get", 1), ("get", 3)])
# A failed get must not change anything.
emit_seq(2, [("put", 1, 1), ("put", 2, 2), ("get", 3), ("put", 3, 3), ("get", 1), ("get", 2)])
emit_seq(3, [("put", MAXKEY, MAXVAL), ("put", 0, 0), ("get", MAXKEY), ("get", 0), ("get", 5)])
emit_seq(3000, [("put", k, k) for k in range(3000)] + [("get", k) for k in range(3000)])
emit_seq(5, [("put", k, k) for k in range(20)] + [("get", k) for k in range(20)])

# Small random sequences: tight key ranges force frequent hits, updates and evictions.
for _ in range(30):
    cap = r.randint(1, 5)
    emit_seq(cap, random_calls(r.randint(5, 40), r.randint(cap, 3 * cap + 2), r.choice([0.3, 0.5, 0.7])))
for _ in range(20):
    cap = r.randint(1, 100)
    emit_seq(cap, random_calls(r.randint(50, 2000), r.randint(cap, 3 * cap), r.choice([0.3, 0.5, 0.7])))

# Large sequences on a full-size cache: O(capacity) per operation is too slow here.
emit_seq(3000, random_calls(10**5, 4000, 0.5, 999))
emit_seq(3000, random_calls(10**5, 3600, 0.6, 999))
emit_seq(3000, random_calls(5 * 10**4, MAXKEY, 0.5))
