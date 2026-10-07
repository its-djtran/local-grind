import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(42)
N = 2 * 10**4
H = 10**5


def brute(h):
    return sum(min(max(h[:i + 1]), max(h[i:])) - h[i] for i in range(len(h)))


def stack_based(h):
    stack, total = [], 0
    for i, x in enumerate(h):
        while stack and h[stack[-1]] < x:
            floor = h[stack.pop()]
            if not stack:
                break
            left = stack[-1]
            total += (min(h[left], x) - floor) * (i - left - 1)
        stack.append(i)
    return total


def put(h):
    assert 1 <= len(h) <= N and all(0 <= x <= H for x in h)
    if len(h) <= 300:
        assert brute(h) == stack_based(h)
    emit(h)


def mountains(n, peaks):
    """Several random peaks with valleys between them."""
    tops = sorted(r.sample(range(n), peaks))
    h = [0] * n
    for t in tops:
        top = r.randint(H // 2, H)
        slope = r.randint(1, max(1, H // max(1, n // peaks)))
        for i in range(n):
            h[i] = max(h[i], top - slope * abs(i - t))
    return [max(0, x - r.randint(0, slope)) for x in h]


for h in [[0], [5], [H], [1, 2], [2, 1], [2, 0, 2], [0, 0, 0], [3, 0, 0, 2, 0, 4], [5, 4, 1, 2],
          [1, 2, 3, 4, 5], [5, 4, 3, 2, 1], [4, 4, 4, 4], [5, 0, 5, 0, 5], [0, 5, 0], [H, 0, H],
          [H, 0, 0, 0, 1], [1, 0, 1, 0, 1, 0, 1], [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0],
          [6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1, 2, 1, 3, 4, 6, 8, 1, 3]]:
    put(h)
for n in range(3, 15):
    put(int_array(r, n, 0, 10))
for _ in range(25):
    n = r.randint(15, 2000)
    put(int_array(r, n, 0, r.choice([3, 100, H])))
for _ in range(3):
    put(mountains(r.randint(100, 2000), r.randint(2, 6)))
# Large cases at the limits.
put(list(range(N)))
put([H - i for i in range(N)])
put([H] + [0] * (N - 2) + [H])
put([H if i % 2 == 0 else 0 for i in range(N)])
put([min(i, N - 1 - i) * 10 for i in range(N)])            # one central peak: no water
put([abs(N // 2 - i) * 10 for i in range(N)])              # one big valley
put(mountains(N, 5))
for _ in range(5):
    put(int_array(r, N, 0, H))
