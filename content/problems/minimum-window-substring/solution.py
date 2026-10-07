class Solution:
    def minWindow(self, s: str, t: str) -> str:
        need = Counter(t)
        missing = len(t)
        left = 0
        best_start, best_len = 0, len(s) + 1
        for right, ch in enumerate(s):
            if need[ch] > 0:
                missing -= 1
            need[ch] -= 1
            if missing == 0:
                # Drop surplus characters from the left; the window stays valid.
                while need[s[left]] < 0:
                    need[s[left]] += 1
                    left += 1
                if right - left + 1 < best_len:
                    best_start, best_len = left, right - left + 1
                # Give up the leftmost required character to look for the next window.
                need[s[left]] += 1
                missing += 1
                left += 1
        return "" if best_len > len(s) else s[best_start:best_start + best_len]
