class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        if len(ransomNote) > len(magazine):
            return False
        counts = [0] * 26
        for ch in magazine:
            counts[ord(ch) - 97] += 1
        for ch in ransomNote:
            i = ord(ch) - 97
            if counts[i] == 0:
                return False
            counts[i] -= 1
        return True
