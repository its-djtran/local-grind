import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, int_array, rng

r = rng(121)
N, HI = 10**5, 10**4

emit([1])
emit([0])
emit([HI])
emit([1, 2])
emit([2, 1])
emit([5, 5, 5, 5])
emit([0, HI])
emit([HI, 0])
emit([3, 2, 6, 5, 0, 3])
emit([2, 4, 1])  # best buy is not the global minimum
emit([1, 2, 4, 2, 5, 7, 2, 4, 9, 0])
emit(list(range(1, 11)))
emit(list(range(10, 0, -1)))
for n in range(2, 15):
    emit(int_array(r, n, 0, 20))
for _ in range(25):
    emit(int_array(r, r.randint(15, 1000), 0, HI))
# Large cases: random, strictly falling (answer 0), rising, and the minimum placed late.
emit(int_array(r, N, 0, HI))
emit(int_array(r, N, 0, HI))
emit([HI - i // 10 for i in range(N)])
emit([i // 10 for i in range(N)])
emit([r.randint(5000, HI) for _ in range(N - 2)] + [0, 4999])
emit([r.randint(5000, HI) for _ in range(N // 2)] + [r.randint(0, 6000) for _ in range(N // 2)])
