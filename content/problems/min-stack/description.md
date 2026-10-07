Design a stack that, in addition to the usual operations, can report its smallest element at any moment.

Implement the `MinStack` class:

- `MinStack()` creates an empty stack.
- `void push(int val)` places `val` on top of the stack.
- `void pop()` discards the element on top of the stack.
- `int top()` returns the element on top of the stack without removing it.
- `int getMin()` returns the smallest element currently in the stack.

Every one of these operations must take `O(1)` time.

The input is a list of operation names and a matching list of argument lists; the first operation constructs the stack. The output lists each operation's return value, with `null` for operations that return nothing.

<!-- examples -->

**Constraints:**

- `-2^31 <= val <= 2^31 - 1`
- `pop`, `top` and `getMin` are only called when the stack is non-empty.
- At most `3 * 10^4` calls are made to `push`, `pop`, `top` and `getMin` in total.
