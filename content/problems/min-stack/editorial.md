## Approach 1: Scan for the Minimum

**Intuition.** Use a plain list as the stack and compute the minimum on demand.

```python
class MinStack:
    def __init__(self):
        self.stack = []

    def push(self, val: int) -> None:
        self.stack.append(val)

    def pop(self) -> None:
        self.stack.pop()

    def top(self) -> int:
        return self.stack[-1]

    def getMin(self) -> int:
        return min(self.stack)
```

- **Time:** `O(1)` for `push`, `pop` and `top`, but `O(n)` for `getMin`. Thousands of `getMin`
  calls on a deep stack add up to `O(n^2)`.
- **Space:** `O(n)`.

## Approach 2: Store the Running Minimum With Each Element (Optimal)

**Intuition.** A stack only changes at the top, so the elements below any given element never
change while it is on the stack. That means "the minimum of everything from the bottom up to here"
is fixed at push time. Store it next to the value; the answer to `getMin` is then always the
minimum stored at the top, and popping automatically restores the previous minimum.

**Algorithm.**

- `push(val)`: append the pair `(val, min(val, current minimum))`, where the current minimum is
  the second field of the top pair (or `val` itself if the stack is empty).
- `pop()`: remove the top pair.
- `top()`: first field of the top pair.
- `getMin()`: second field of the top pair.

```python
class MinStack:
    def __init__(self):
        self.stack = []  # (value, minimum of the stack up to and including this value)

    def push(self, val: int) -> None:
        current_min = min(val, self.stack[-1][1]) if self.stack else val
        self.stack.append((val, current_min))

    def pop(self) -> None:
        self.stack.pop()

    def top(self) -> int:
        return self.stack[-1][0]

    def getMin(self) -> int:
        return self.stack[-1][1]
```

- **Time:** `O(1)` for every operation.
- **Space:** `O(n)` — one extra number per element.

A variant keeps a second stack holding only values that are `<=` the current minimum, which
saves space when the minimum rarely changes. Pushing on equality matters: with duplicates of the
minimum, popping one copy must not lose the others.
