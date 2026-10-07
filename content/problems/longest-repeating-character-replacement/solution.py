class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        count = [0] * 26
        left = maxf = 0
        for right in range(len(s)):
            c = ord(s[right]) - 65
            count[c] += 1
            if count[c] > maxf:
                maxf = count[c]
            # The window never shrinks: if it is too costly, slide it right by one instead.
            if right - left + 1 - maxf > k:
                count[ord(s[left]) - 65] -= 1
                left += 1
        return len(s) - left
