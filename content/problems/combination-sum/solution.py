class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates = sorted(candidates)
        result, combo = [], []

        def backtrack(start, remaining):
            if remaining == 0:
                result.append(combo[:])
                return
            for i in range(start, len(candidates)):
                c = candidates[i]
                if c > remaining:
                    break
                combo.append(c)
                backtrack(i, remaining - c)
                combo.pop()

        backtrack(0, target)
        return result
