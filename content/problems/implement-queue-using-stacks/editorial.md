## Approach 1: Keep the Front on Top (Costly Push)

**Intuition.** If the oldest element were always on top of the main stack, `pop` and `peek` would
be ordinary stack operations. To put a new element at the *bottom*, move everything to a helper
stack, push the new element, and move everything back.

```python
class MyQueue:
    def __init__(self):
        self.main = []    # top of the stack = front of the queue
        self.helper = []

    def push(self, x: int) -> None:
        while self.main:
            self.helper.append(self.main.pop())
        self.main.append(x)
        while self.helper:
            self.main.append(self.helper.pop())

    def pop(self) -> int:
        return self.main.pop()

    def peek(self) -> int:
        return self.main[-1]

    def empty(self) -> bool:
        return not self.main
```

- **Time:** `O(n)` per `push`; `O(1)` for the other operations.
- **Space:** `O(n)`.

## Approach 2: Inbox and Outbox Stacks (Optimal)

**Intuition.** Pushes go onto an `inbox` stack. Pouring the inbox into an `outbox` stack reverses
it, so the oldest element ends up on top of the outbox. The key observation is that once elements
are in the outbox, they are already in the right order for every future `pop`, so there is no
reason to touch them again. Only refill the outbox when it runs empty.

**Algorithm.**

- `push(x)`: push onto `inbox`.
- `peek()`: if `outbox` is empty, pop every element of `inbox` onto `outbox`. Return the top of
  `outbox`.
- `pop()`: call `peek()` to make sure the outbox is filled, then pop from `outbox`.
- `empty()`: both stacks are empty.

Refilling while the outbox still holds elements would bury the oldest elements under newer
ones and break the FIFO order.

```python
class MyQueue:
    def __init__(self):
        self.inbox = []
        self.outbox = []

    def push(self, x: int) -> None:
        self.inbox.append(x)

    def pop(self) -> int:
        self.peek()
        return self.outbox.pop()

    def peek(self) -> int:
        if not self.outbox:
            while self.inbox:
                self.outbox.append(self.inbox.pop())
        return self.outbox[-1]

    def empty(self) -> bool:
        return not self.inbox and not self.outbox
```

- **Time:** amortized `O(1)` per operation. A single `peek` or `pop` may move many elements, but
  each element is moved from `inbox` to `outbox` at most once in its lifetime, so `n` operations
  cost `O(n)` in total.
- **Space:** `O(n)`.
