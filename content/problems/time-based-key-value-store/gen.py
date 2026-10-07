import bisect
import string
import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(981)
T = 10**7
ALPHA = string.ascii_lowercase + string.digits


class Naive:
    def __init__(self):
        self.log = []

    def set(self, key, value, t):
        self.log.append((key, value, t))

    def get(self, key, t):
        best = ""
        for k, v, s in self.log:
            if k == key and s <= t:
                best = v
        return best


class Fast:
    def __init__(self):
        self.times, self.values = {}, {}

    def set(self, key, value, t):
        self.times.setdefault(key, []).append(t)
        self.values.setdefault(key, []).append(value)

    def get(self, key, t):
        i = bisect.bisect_right(self.times.get(key, []), t)
        return self.values[key][i - 1] if i else ""


def put(ops, args):
    assert ops[0] == "TimeMap" and args[0] == [] and len(ops) == len(args)
    assert len(ops) - 1 <= 2 * 10**5
    last = 0
    for op, a in zip(ops[1:], args[1:]):
        assert 1 <= len(a[0]) <= 100 and set(a[0]) <= set(ALPHA)
        assert 1 <= a[-1] <= T
        if op == "set":
            assert 1 <= len(a[1]) <= 100 and set(a[1]) <= set(ALPHA)
            assert a[2] > last
            last = a[2]
    if len(ops) <= 2000:  # cross-check a binary-search model against a naive one
        naive, fast = Naive(), Fast()
        for op, a in zip(ops[1:], args[1:]):
            assert getattr(naive, op)(*a) == getattr(fast, op)(*a)
    emit(ops, args)


def scenario(n_ops, keys, set_ratio, val_len=(1, 8), max_step=50, t0=0, unseen_ratio=0.05):
    """A random mix of sets (strictly increasing times) and gets of varied times."""
    ops, args = ["TimeMap"], [[]]
    t, set_times = t0, []
    room = T - t0
    step_cap = max(1, min(max_step, room // max(1, n_ops)))
    for _ in range(n_ops):
        if r.random() < set_ratio and t + step_cap <= T:
            t += r.randint(1, step_cap)
            set_times.append(t)
            ops.append("set")
            args.append([r.choice(keys), random_string(r, r.randint(*val_len), ALPHA), t])
        else:
            key = r.choice(keys) if r.random() >= unseen_ratio else "zz" + random_string(r, 3, ALPHA)
            roll = r.random()
            if set_times and roll < 0.3:
                q = r.choice(set_times)                      # exactly a set time
            elif set_times and roll < 0.5:
                q = max(1, r.choice(set_times) - 1)          # just before a set time
            elif roll < 0.6:
                q = r.randint(1, max(1, t0 + 1))             # before everything
            elif roll < 0.7:
                q = T                                        # far future
            else:
                q = r.randint(1, max(1, t + 10))
            ops.append("get")
            args.append([key, q])
    return ops, args


ex_ops = ["TimeMap", "set", "get", "get", "set", "get", "get"]
put(ex_ops, [[], ["foo", "bar", 1], ["foo", 1], ["foo", 3], ["foo", "bar2", 4], ["foo", 4], ["foo", 5]])
put(["TimeMap", "get"], [[], ["a", 1]])
put(["TimeMap", "set", "get", "get", "get"], [[], ["a", "x", 5], ["a", 4], ["a", 5], ["b", 5]])
put(["TimeMap", "set", "set", "get", "get", "get"],
    [[], ["k", "v1", 1], ["k", "v2", T], ["k", T - 1], ["k", T], ["k", 1]])
put(["TimeMap", "set", "set", "set", "get", "get", "get", "get"],
    [[], ["love", "high", 10], ["love", "low", 20], ["hate", "x", 30],
     ["love", 5], ["love", 10], ["love", 15], ["love", 25]])
put(["TimeMap", "set", "get", "set", "get", "get"],
    [[], ["a" * 100, "b" * 100, 1], ["a" * 100, 1], ["a" * 99, "c" * 100, 2], ["a" * 100, 2], ["a" * 99, 1]])
put(["TimeMap", "set", "set", "get", "get"],
    [[], ["1", "0", 3], ["10", "1", 7], ["1", 7], ["10", 6]])
for _ in range(30):
    put(*scenario(r.randint(1, 30), [random_string(r, r.randint(1, 3), "ab1") for _ in range(r.randint(1, 3))],
                  r.random(), max_step=r.choice([1, 3, 10])))
for _ in range(20):
    keys = [random_string(r, r.randint(1, 10), ALPHA) for _ in range(r.randint(1, 20))]
    put(*scenario(r.randint(30, 1500), keys, r.uniform(0.3, 0.7), val_len=(1, 20),
                  max_step=r.choice([1, 100, 10**4]), t0=r.choice([0, 0, 10**6])))
for _ in range(3):
    keys = [random_string(r, r.randint(90, 100), ALPHA) for _ in range(5)]
    put(*scenario(400, keys, 0.5, val_len=(90, 100), max_step=10**4))

# Large cases: long histories queried at old times defeat a linear scan.
n = 15000
ops = ["TimeMap"] + ["set"] * n + ["get"] * n
args = [[]] + [["k", random_string(r, 2, ALPHA), 3 * i + 1] for i in range(n)]
args += [["k", r.randint(1, (3 * n + 5) // r.choice([1, 10]))] for _ in range(n)]
put(ops, args)
put(*scenario(40000, ["a", "b"], 0.4, val_len=(1, 3), max_step=100))
put(*scenario(30000, ["k"], 0.5, val_len=(1, 3), max_step=200, t0=T - 30000 * 200))
put(*scenario(20000, [random_string(r, 4, ALPHA) for _ in range(2000)], 0.5, val_len=(1, 3), max_step=300))
