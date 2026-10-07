A CPU must run a list of `tasks`, each labelled by an uppercase letter from `A` to `Z`. Time is divided into equal slots; in each slot the CPU either runs exactly one task or sits idle. Tasks may be run in any order.

There is a cooldown rule: after a task with some label runs, at least `n` slots must pass before another task with the **same label** can run. Put differently, two tasks with the same label must be more than `n` slots apart.

Return the **fewest slots** needed to finish every task.

<!-- examples -->

**Constraints:**

- `1 <= tasks.length <= 10^4`
- `tasks[i]` is an uppercase English letter.
- `0 <= n <= 100`
