import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(70)

# Every legal input once, smallest first.
for n in range(1, 46):
    emit(n)
# The largest inputs again in shuffled order: an exponential recursion cannot finish these.
big = list(range(36, 46))
r.shuffle(big)
for n in big:
    emit(n)
