import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(17)
DIGITS = "23456789"
emitted = set()


def case(digits):
    assert 0 <= len(digits) <= 4 and all(d in DIGITS for d in digits)
    emitted.add(digits)
    emit(digits)


# Edge cases: empty input, every single digit (including the four-letter 7 and 9), extremes.
case("")
for d in DIGITS:
    case(d)
case("79")
case("97")
case("7777")
case("9999")
case("2222")
case("7979")
case("22")
case("234")
case("5678")
case("9872")

# Random inputs of every length, mostly of the maximum length 4.
while len(emitted) < 40:
    case(random_string(r, r.randint(2, 3), DIGITS))
while len(emitted) < 75:
    s = random_string(r, 4, DIGITS)
    if s not in emitted:
        case(s)
