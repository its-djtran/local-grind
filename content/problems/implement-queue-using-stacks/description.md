Build a first-in-first-out (FIFO) queue using nothing but **two stacks**.

Implement the `MyQueue` class:

- `MyQueue()` creates an empty queue.
- `void push(int x)` adds `x` to the back of the queue.
- `int pop()` removes the element at the front of the queue and returns it.
- `int peek()` returns the element at the front of the queue without removing it.
- `boolean empty()` returns `true` if the queue holds no elements and `false` otherwise.

Restrict yourself to standard stack operations: push onto the top, look at or pop from the top, ask for the size, and check whether a stack is empty. In Python, use a list and touch only its end (`append`, `pop()`, `[-1]`, `len`); a `deque` or `pop(0)` would sidestep the exercise.

The input is a list of operation names and a matching list of argument lists; the first operation constructs the queue. The output lists each operation's return value, with `null` for operations that return nothing.

<!-- examples -->

**Constraints:**

- `1 <= x <= 9`
- At most `100` calls are made to `push`, `pop`, `peek` and `empty`.
- `pop` and `peek` are only called when the queue is non-empty.

**Follow-up:** Can you make every operation run in **amortized** `O(1)` time, so that any sequence of `n` operations takes `O(n)` time overall, even if a single operation is occasionally slower?
