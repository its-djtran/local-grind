import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(271)
ALL = [chr(c) for c in range(256)]
TRICKY = list("#,;:|/\\\"' \t\n\r0123456789") + ["\x00", "\x1e", "\x7f", "\xff", "\x85", "\xa0"]


def put(strs):
    assert 1 <= len(strs) <= 200
    assert all(len(s) <= 200 and all(ord(c) < 256 for c in s) for s in strs)
    emit(strs)


def rand_str(n, alphabet):
    return "".join(r.choice(alphabet) for _ in range(n))


put(["Hello", "World"])
put([""])
put(["", ""])
put(["", "a", ""])
put(["#"])
put(["0#", "1#a", "#2#"])
put(["5#Hello", "5#World"])
put(["3", "#", "abc"])
put([",", ",,", ""])
put(["a,b", "c;d", "e|f"])
put(["/", "//", "/:", ":/"])
put(["\\", "\\\\", "\\n", "\n"])
put(['"', "'", '""', "[]", "{}", "null"])
put(["\x00", "\x00\x00", "a\x00b"])
put([" ", "  ", "\t", "\r\n"])
put(["12", "345", "6789"])
put(["10#abcdefghij"])
put(["9" * 200])
put(["#" * 200, "#" * 199])
put([chr(c) for c in range(200)])
put([chr(c) for c in range(56, 256)])
put(["".join(ALL[:200]), "".join(ALL[200:]) + "".join(ALL[:144])])
put(["".join(reversed(ALL[56:]))])
put([""] * 200)
put(["a"] * 200)
# Every one of the 256 characters as a one-character string, split over two tests.
put(ALL[:128])
put(ALL[128:])
for n in range(1, 11):
    put([rand_str(r.randint(0, 6), TRICKY) for _ in range(n)])
for _ in range(25):
    k = r.randint(1, 50)
    put([rand_str(r.randint(0, 30), r.choice([ALL, TRICKY])) for _ in range(k)])
for _ in range(10):
    put([rand_str(r.randint(0, 200), ALL) for _ in range(r.randint(50, 200))])
# Largest inputs: 200 strings of 200 characters.
put([rand_str(200, ALL) for _ in range(200)])
put([rand_str(200, TRICKY) for _ in range(200)])
put([rand_str(200, "#0123456789") for _ in range(200)])
