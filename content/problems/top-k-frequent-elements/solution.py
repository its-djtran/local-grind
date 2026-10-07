class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count = Counter(nums)
        buckets = [[] for _ in range(len(nums) + 1)]
        for value, c in count.items():
            buckets[c].append(value)
        result = []
        for c in range(len(nums), 0, -1):
            for value in buckets[c]:
                result.append(value)
                if len(result) == k:
                    return result
        return result
