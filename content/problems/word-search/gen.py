import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(79)
LETTERS = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


def board(m, n, alphabet):
    return [[r.choice(alphabet) for _ in range(n)] for _ in range(m)]


def fill(m, n, ch):
    return [[ch] * n for _ in range(m)]


def path_word(b, length):
    """Spells a random self-avoiding path, so the answer is true. Falls back to a shorter path."""
    m, n = len(b), len(b[0])
    i, j = r.randrange(m), r.randrange(n)
    seen, out = {(i, j)}, [b[i][j]]
    while len(out) < length:
        nbrs = [(i + di, j + dj) for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1))
                if 0 <= i + di < m and 0 <= j + dj < n and (i + di, j + dj) not in seen]
        if not nbrs:
            break
        i, j = r.choice(nbrs)
        seen.add((i, j))
        out.append(b[i][j])
    return "".join(out)


def random_case(m, n, alphabet, want_true):
    b = board(m, n, alphabet)
    length = r.randint(1, min(15, m * n + 1))
    if want_true:
        return b, path_word(b, length)
    return b, "".join(r.choice(alphabet) for _ in range(length))


ex = [["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]]
emit([["a"]], "a")
emit([["a"]], "b")
emit([["a"]], "A")
emit([["a"]], "aa")
emit([["a", "b"]], "ba")
emit([["a", "b"], ["c", "d"]], "abcd")
emit([["a", "b"], ["c", "d"]], "abdc")
emit([["a", "b"], ["c", "d"]], "acdb")
emit([["a", "b"], ["c", "d"]], "ad")
emit(ex, "ABCESEEEFS")
emit(ex, "ABCESCFSADEE")
emit(ex, "SFCS")
emit(ex, "ECCE")
emit([["C", "A", "A"], ["A", "A", "A"], ["B", "C", "D"]], "AAB")
emit([["A", "B", "C", "E"], ["S", "F", "E", "S"], ["A", "D", "E", "E"]], "ABCESEEEFS")
emit([["a", "a"]], "aaa")
emit(fill(6, 6, "z"), "z" * 15)
emit([list("abcdef"), list("ghijkl"), list("mnopqr"), list("stuvwx"), list("yzABCD"), list("EFGHIJ")],
     "abcdefljihgmnop")
for _ in range(30):
    m, n = r.randint(1, 4), r.randint(1, 4)
    emit(*random_case(m, n, r.choice(["ab", "abc", "aAb"]), r.random() < 0.5))
for _ in range(25):
    m, n = r.randint(3, 6), r.randint(3, 6)
    emit(*random_case(m, n, r.choice(["ab", "abc", "abcd", "aAbB", LETTERS]), r.random() < 0.5))

# Adversarial boards: plain backtracking explores millions of paths before failing.
emit(fill(6, 6, "A"), "A" * 14 + "B")
emit(fill(6, 6, "a"), "a" * 14 + "A")
emit(fill(6, 6, "A"), "B" + "A" * 14)
b = fill(6, 6, "A")
b[5][5] = "B"
emit(b, "A" * 14 + "B")
b = fill(6, 6, "A")
b[0][0] = "B"
emit(b, "B" + "A" * 13 + "C")
b = fill(6, 6, "a")
b[0][0] = "b"
b[0][1] = "b"
emit(b, "a" * 13 + "bb")
b = fill(6, 6, "A")
b[3][3] = "B"
emit(b, "A" * 7 + "B" + "A" * 7)
emit([["a", "b"] * 3 if i % 2 == 0 else ["b", "a"] * 3 for i in range(6)], "ab" * 7 + "c")
emit(fill(6, 6, "X"), "X" * 14 + "Y")
emit(fill(6, 6, "z"), "z" * 14 + "Z")
# Letter counts are sufficient here, so only searching from the rarer end avoids the blow-up.
for a, c in (("A", "B"), ("q", "Q")):
    b = fill(6, 6, a)
    b[0][0] = c
    b[5][5] = c
    emit(b, a * 13 + c * 2)
