import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, random_string, rng

r = rng(424)
UP = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
N = 10**5

emit("A", 0)
emit("A", 1)
emit("AB", 0)
emit("AB", 2)
emit("AAAA", 0)
emit("ABCDE", 0)
emit("ABCDE", 1)
emit("ABCDE", 5)
emit("ABBB", 2)
emit("BAAAB", 2)
emit("AAAB", 0)
emit("ABAA", 0)
emit("AABA", 0)
emit("ABCABCABC", 2)
emit("KRSCDCSONAJNHLBMDQGIFCPEKPOHQIHLTDIQGEKLRLCQNBOHNDQGHJPNDQPERNFSSSRDEQLFPCCCARFMDLHADJADAGNNSBNCJQOF", 4)
for n in range(2, 14):
    emit(random_string(r, n, "AB"), r.randint(0, n))
for _ in range(20):
    n = r.randint(5, 300)
    emit(random_string(r, n, r.choice(["AB", "ABC", "ABCDEF", UP])), r.randint(0, n // 3))
for _ in range(8):
    n = r.randint(1000, 10000)
    emit(random_string(r, n, r.choice(["ABC", UP])), r.randint(0, 50))
emit("A" * N, 0)
emit("A" * N, N)
emit(random_string(r, N, UP), 0)
emit(random_string(r, N, UP), N)
emit((UP * (N // 26 + 1))[:N], 3)
# Large k means long answers, so growing a window from every start costs ~n^2/2 steps.
for _ in range(4):
    emit(random_string(r, N, r.choice(["AB", "ABC", UP])), r.randint(N // 4, N // 2))
