## Approach 1: Collapse the First Operator Repeatedly

**Intuition.** In a valid postfix expression, the leftmost operator always applies to the two
tokens right before it, and both of those are plain numbers. Replace those three tokens with
their result and repeat until one token is left.

```python
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        def apply(a, b, op):
            if op == "+":
                return a + b
            if op == "-":
                return a - b
            if op == "*":
                return a * b
            return int(a / b)

        tokens = list(tokens)
        while len(tokens) > 1:
            i = 0
            while tokens[i] not in "+-*/":
                i += 1
            value = apply(int(tokens[i - 2]), int(tokens[i - 1]), tokens[i])
            tokens[i - 2 : i + 1] = [str(value)]
        return int(tokens[0])
```

(The membership test `tokens[i] not in "+-*/"` is safe for numbers such as `"-11"` because it
checks for a substring, and no multi-character number is a substring of `"+-*/"`.)

- **Time:** `O(n^2)` — each collapse rescans from the start and shifts the rest of the list.
  When all the operators sit at the end, each scan covers about half the tokens.
- **Space:** `O(n)` for the working copy.

## Approach 2: Operand Stack (Optimal)

**Intuition.** Scanning left to right, the values waiting for an operator are exactly the ones
seen most recently, so a stack holds them in the right order. Each operator consumes the top two
values and produces one.

**Algorithm.**

1. For each token: if it is an operator, pop `b` (the right operand) and then `a` (the left
   operand), and push `a op b`. Otherwise push `int(token)`.
2. The single value left on the stack is the answer.

Two Python details: pop order matters for `-` and `/`, and Python's `a // b` rounds toward
negative infinity (`-7 // 2 == -4`), whereas the problem asks for truncation toward zero
(`-3`). `int(a / b)` truncates and is exact here because all values fit in 32 bits, well within
a float's 53-bit precision.

```python
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        for token in tokens:
            if token in ("+", "-", "*", "/"):
                b = stack.pop()
                a = stack.pop()
                if token == "+":
                    stack.append(a + b)
                elif token == "-":
                    stack.append(a - b)
                elif token == "*":
                    stack.append(a * b)
                else:
                    stack.append(int(a / b))
            else:
                stack.append(int(token))
        return stack[0]
```

- **Time:** `O(n)` — each token is handled once.
- **Space:** `O(n)` for the stack.
