class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        keypad = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl",
                  "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}
        combos = [""]
        for d in digits:
            combos = [c + letter for c in combos for letter in keypad[d]]
        return combos
