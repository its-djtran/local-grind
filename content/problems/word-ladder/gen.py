import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(127)
ALPHA = "abcdefghijklmnopqrstuvwxyz"


def word(L, alpha):
    return "".join(r.choice(alpha) for _ in range(L))


def mutate(w, alpha):
    i = r.randrange(len(w))
    c = r.choice([x for x in alpha if x != w[i]])
    return w[:i] + c + w[i + 1:]


def ladder_brute(begin, end, words):
    """Plain BFS with pairwise comparisons (independent of the reference)."""
    if end not in words:
        return 0
    dist = {begin: 1}
    frontier = [begin]
    while frontier:
        nxt = []
        for u in frontier:
            for w in words:
                if w not in dist and sum(a != b for a, b in zip(u, w)) == 1:
                    dist[w] = dist[u] + 1
                    nxt.append(w)
        frontier = nxt
    return dist.get(end, 0)


def random_case(L, alpha, n, end_in_list=0.85):
    while True:
        words = sorted({word(L, alpha) for _ in range(n)})
        begin = word(L, alpha)
        if r.random() < end_in_list:
            end = r.choice(words)
        else:
            end = word(L, alpha)
        if begin != end:
            r.shuffle(words)
            return begin, end, words


def walk_case(L, steps, noise, alpha=ALPHA, end_at_last=True):
    """A random walk of single-letter changes (a long ladder) mixed with random noise words."""
    while True:
        cur = word(L, alpha)
        path = [cur]
        seen = {cur}
        for _ in range(steps):
            for _ in range(50):
                nxt = mutate(cur, alpha)
                if nxt not in seen:
                    break
            else:
                break
            seen.add(nxt)
            path.append(nxt)
            cur = nxt
        words = set(path[1:])
        target = min(5000, len(path) - 1 + noise, len(alpha) ** L - 1)
        while len(words) < target:
            w = word(L, alpha)
            if w != path[0]:
                words.add(w)
        words = sorted(words)
        end = path[-1] if end_at_last else r.choice(words)
        if end in words and end != path[0]:
            r.shuffle(words)
            return path[0], end, words


emit("a", "c", ["a", "b", "c"])
emit("a", "b", ["b"])
emit("a", "b", ["c"])
emit("hot", "dog", ["hot", "dog"])
emit("hot", "dog", ["hot", "dot", "dog"])
emit("hit", "hot", ["hot"])
emit("hit", "cog", ["hot", "dot", "dog", "lot", "log", "cog", "hit"])
emit("abc", "xyz", ["xyz"])
emit("lost", "cost", ["most", "fist", "lost", "cost", "fish"])
emit("leet", "code", ["lest", "leet", "lose", "code", "lode", "robe", "lost"])
emit("ab", "ba", ["aa", "bb", "ba"])
emit("aaaaaaaaaa", "bbbbbbbbbb", ["baaaaaaaaa", "bbaaaaaaaa", "bbbaaaaaaa", "bbbbaaaaaa", "bbbbbaaaaa",
                                 "bbbbbbaaaa", "bbbbbbbaaa", "bbbbbbbbaa", "bbbbbbbbba", "bbbbbbbbbb"])
for _ in range(25):
    L = r.randint(1, 4)
    alpha = ALPHA[:r.randint(2, 6)]
    emit(*random_case(L, alpha, r.randint(1, 30)))
for _ in range(10):
    emit(*walk_case(r.randint(3, 8), r.randint(3, 30), r.randint(0, 40), ALPHA[:r.randint(3, 8)]))
for _ in range(5):
    L = r.randint(3, 6)
    emit(*random_case(L, ALPHA[:r.randint(3, 6)], r.randint(200, 1000)))

# Large: 5000 words. Long ladders force BFS through most of the dictionary, so pairwise
# comparisons (N^2 * L) are far too slow.
emit(*walk_case(10, 4999, 0))
emit(*walk_case(10, 2500, 2500))
emit(*walk_case(5, 3000, 2000, ALPHA[:6]))
emit(*walk_case(8, 2000, 3000, ALPHA[:4]))
three = [a + b + c for a in ALPHA[:17] for b in ALPHA[:17] for c in ALPHA[:17]][:5000]
r.shuffle(three)
emit("zab", three[0], three)
emit(*random_case(4, ALPHA[:9], 5000))
words = sorted({word(10, "ab") for _ in range(900)} - {"a" * 10})
emit("a" * 10, "b" * 10, words + (["b" * 10] if "b" * 10 not in words else []))
emit(*walk_case(10, 4000, 1000, end_at_last=False))
