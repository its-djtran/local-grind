class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        m = len(p)
        if m > len(s):
            return []
        diff = [0] * 26  # window count minus p count, per letter
        for ch in p:
            diff[ord(ch) - 97] -= 1
        codes = [ord(ch) - 97 for ch in s]
        for c in codes[:m]:
            diff[c] += 1
        bad = sum(1 for d in diff if d != 0)  # letters whose counts disagree
        out = [0] if bad == 0 else []
        for i in range(m, len(codes)):
            add, rem = codes[i], codes[i - m]
            if add != rem:
                d = diff[add]
                bad += (d == 0) - (d == -1)
                diff[add] = d + 1
                d = diff[rem]
                bad += (d == 0) - (d == 1)
                diff[rem] = d - 1
            if bad == 0:
                out.append(i - m + 1)
        return out
