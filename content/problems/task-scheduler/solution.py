class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        counts = Counter(tasks).values()
        f = max(counts)
        k = sum(1 for c in counts if c == f)
        return max(len(tasks), (f - 1) * (n + 1) + k)
