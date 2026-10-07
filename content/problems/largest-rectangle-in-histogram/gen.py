import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(84)
N = 10**5
H = 10**4


def brute(h):
    best = 0
    for i in range(len(h)):
        low = h[i]
        for j in range(i, len(h)):
            low = min(low, h[j])
            best = max(best, low * (j - i + 1))
    return best


def stack_based(h):
    stack, best = [], 0
    for i, x in enumerate(h + [0]):
        while stack and h[stack[-1]] >= x:
            height = h[stack.pop()]
            left = stack[-1] if stack else -1
            best = max(best, height * (i - left - 1))
        stack.append(i)
    return best


def put(h):
    assert 1 <= len(h) <= N and all(0 <= x <= H for x in h)
    if len(h) <= 300:
        assert brute(h) == stack_based(h)
    emit(h)


for h in [[0], [1], [H], [2, 4], [4, 2], [0, 0], [1, 1], [0, 9], [2, 1, 2], [1, 2, 1],
          [3, 3, 3, 3], [1, 2, 3, 4, 5], [5, 4, 3, 2, 1], [2, 1, 5, 6, 2, 3, 0], [6, 2, 5, 4, 5, 1, 6],
          [4, 2, 0, 3, 2, 5], [H, 0, H], [1, H, 1], [5, 5, 1, 7, 1, 1, 5, 2, 7, 6], [0, 2, 0, 2, 0]]:
    put(h)
for n in range(3, 15):
    put(int_array(r, n, 0, 10))
for _ in range(25):
    n = r.randint(15, 3000)
    put(int_array(r, n, 0, r.choice([2, 10, 1000, H])))
# Large cases at the limits.
put([i * H // N for i in range(N)])                 # non-decreasing: worst case for divide and conquer
put([(N - 1 - i) * H // N for i in range(N)])       # non-increasing
put([H] * N)
put([1 + (i % 2) for i in range(N)])
put([H - abs(N // 2 - i) * H // N for i in range(N)])  # single mountain
for _ in range(2):
    put(int_array(r, N, 0, H))
