import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(322)
MAXC = 2**31 - 1

# Edge cases.
emit([1], 0)
emit([7, 3], 0)
emit([1], 1)
emit([2], 1)
emit([1], 10**4)
emit([MAXC], 10**4)
emit([MAXC, 1], 2)
emit([MAXC, 10**4], 10**4)
emit([1, 3, 4], 6)           # greedy takes 4+1+1, optimum is 3+3
emit([1, 5, 6, 9], 11)       # greedy takes 9+1+1, optimum is 5+6
emit([186, 419, 83, 408], 6249)
emit([3, 7], 11)
emit([3, 7], 8)
emit([5, 5, 5, 2], 9)        # duplicate denominations
emit([2, 4, 6, 8], 9999)     # all even, odd target: impossible
emit([10**4], 9999)
emit([10**4 + 1, 10**5], 10**4)

# Small random cases.
for _ in range(30):
    k = r.randint(1, 6)
    coins = [r.randint(1, 25) for _ in range(k)]
    emit(coins, r.randint(0, 120))

# Medium random cases, including huge denominations that never fit.
for _ in range(15):
    k = r.randint(1, 12)
    coins = [r.choice([r.randint(1, 500), r.randint(1, MAXC)]) for _ in range(k)]
    emit(coins, r.randint(0, 3000))

# Large cases at the limits.
for _ in range(4):
    coins = r.sample(range(1, 400), 12)
    emit(coins, 10**4)
emit(list(range(1, 13)), 10**4)
emit([2 * x for x in r.sample(range(50, 900), 12)], 9999)    # impossible, all amounts explored
emit([r.randint(500, 3000) for _ in range(12)], 10**4 - r.randint(0, 50))
emit([997, 991, 983, 977, 971, 967, 953, 947, 941, 937, 929, 919], 10**4)
emit([1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000], 9999)
