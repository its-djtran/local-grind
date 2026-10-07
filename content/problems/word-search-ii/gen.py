import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(212)
LOWER = "abcdefghijklmnopqrstuvwxyz"


def board_of(m, n, alphabet):
    return [[r.choice(alphabet) for _ in range(n)] for _ in range(m)]


def path_word(board, length):
    """Reads a word along a random self-avoiding walk (may stop early at a dead end)."""
    m, n = len(board), len(board[0])
    i, j = r.randrange(m), r.randrange(n)
    seen = {(i, j)}
    out = [board[i][j]]
    while len(out) < length:
        nxt = [(x, y) for x, y in ((i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1))
               if 0 <= x < m and 0 <= y < n and (x, y) not in seen]
        if not nxt:
            break
        i, j = r.choice(nxt)
        seen.add((i, j))
        out.append(board[i][j])
    return "".join(out)


def make_words(board, count, alphabet, found_frac, max_len=10):
    words, seen = [], set()
    attempts = 0
    while len(words) < count and attempts < count * 50:
        attempts += 1
        length = r.randint(1, max_len)
        if r.random() < found_frac:
            w = path_word(board, length)
        else:
            w = random_string(r, length, alphabet)
        if w not in seen:
            seen.add(w)
            words.append(w)
    return words


def case(board, words):
    m, n = len(board), len(board[0])
    assert 1 <= m <= 12 and 1 <= n <= 12 and all(len(row) == n for row in board)
    assert 1 <= len(words) <= 3 * 10**4 and len(set(words)) == len(words)
    assert all(1 <= len(w) <= 10 and w.isalpha() and w.islower() for w in words)
    emit(board, words)


# Edge cases.
case([["a"]], ["a"])
case([["a"]], ["b"])
case([["a"]], ["a", "aa", "b"])
case([["a", "a"]], ["aaa", "aa", "a"])
case([["a", "b"], ["c", "d"]], ["abdc", "acdb", "abcd", "ab", "da", "bd", "dcab"])
case([["a", "b"], ["c", "d"]], ["abcb"])
case([["a", "b", "c"], ["a", "e", "d"], ["a", "f", "g"]], ["abcdefg", "gfedcbaaa", "eaabcdgfa", "befa", "dgc", "ade"])
case([["o", "a", "a", "n"], ["e", "t", "a", "e"], ["i", "h", "k", "r"], ["i", "f", "l", "v"]],
     ["oath", "oat", "oa", "o", "eat", "eata", "pea", "rain", "hklf", "hf"])
case([["a", "b", "c", "e"], ["s", "f", "c", "s"], ["a", "d", "e", "e"]], ["abcced", "see", "abcb", "sfcs", "adfb"])
case([list("abcdefghijkl")], ["abc", "cba", "jkl", "abcdefghij", "lkjihgfedc", "ac", "z"])
case([[c] for c in "abcdefghijkl"], ["abc", "lkj", "efghijkl", "ba", "aa"])
case([["a"] * 12 for _ in range(12)], ["a" * k for k in range(1, 11)])

# Small random boards and word lists.
for _ in range(30):
    m, n = r.randint(1, 5), r.randint(1, 5)
    alphabet = r.choice(["ab", "abc", "abcd", LOWER[:8], LOWER])
    board = board_of(m, n, alphabet)
    case(board, make_words(board, r.randint(1, 15), alphabet, 0.6, r.randint(2, 10)))
for _ in range(12):
    m, n = r.randint(4, 12), r.randint(4, 12)
    alphabet = r.choice(["abc", "abcde", LOWER[:10], LOWER])
    board = board_of(m, n, alphabet)
    case(board, make_words(board, r.randint(20, 2000), alphabet, 0.5))

# Large cases: a full board and the maximum number of words. Searching for each word separately
# repeats the same board exploration tens of thousands of times.
for alphabet, found in [(LOWER, 0.5), (LOWER[:6], 0.3)]:
    board = board_of(12, 12, alphabet)
    case(board, make_words(board, 3 * 10**4, alphabet, found))
# Few distinct letters and long words: each separate search wanders deep into the board before
# failing, while the shared trie explores every useful path once.
for alphabet, lo in [("abc", 8), ("abcd", 7), ("abcd", 7), ("abcde", 6)]:
    board = board_of(12, 12, alphabet)
    words = set()
    while len(words) < 3 * 10**4:
        words.add(random_string(r, r.randint(lo, 10), alphabet))
    ordered = sorted(words)
    r.shuffle(ordered)
    case(board, ordered)
# A board of one letter: the all-'a' words are found at once, while 30000 other words never
# match a single cell. Only the first letters need checking if the words share one trie.
board = [["a"] * 12 for _ in range(12)]
words = ["a" * k for k in range(1, 11)]
words += make_words(board_of(1, 1, "b"), 3 * 10**4 - 10, LOWER[1:], 0.0)
case(board, words)
# A uniform board where every word is found: once found, the words must leave the trie, or the
# search keeps enumerating every path of length 10 on the board.
board = [["z"] * 12 for _ in range(12)]
case(board, ["z" * k for k in range(10, 0, -1)])
