class Solution:
    def calculate(self, s: str) -> int:
        total, sign, num = 0, 1, 0
        stack = []
        for ch in s:
            if ch.isdigit():
                num = num * 10 + ord(ch) - 48
            elif ch == "+" or ch == "-":
                total += sign * num
                num = 0
                sign = 1 if ch == "+" else -1
            elif ch == "(":
                stack.append((total, sign))
                total, sign = 0, 1
            elif ch == ")":
                total += sign * num
                num = 0
                prev, prev_sign = stack.pop()
                total = prev + prev_sign * total
        return total + sign * num
