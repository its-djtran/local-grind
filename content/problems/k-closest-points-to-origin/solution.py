class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        cand = [(x * x + y * y, x, y) for x, y in points]
        result = []
        need = k
        while need > 0:
            a, b, c = cand[0][0], cand[len(cand) // 2][0], cand[-1][0]
            p = sorted((a, b, c))[1]  # median of three
            lo = [c for c in cand if c[0] < p]
            eq = [c for c in cand if c[0] == p]
            if need <= len(lo):
                cand = lo
                continue
            result += lo
            need -= len(lo)
            if need <= len(eq):
                result += eq[:need]
                break
            result += eq
            need -= len(eq)
            cand = [c for c in cand if c[0] > p]
        return [[x, y] for _, x, y in result]
