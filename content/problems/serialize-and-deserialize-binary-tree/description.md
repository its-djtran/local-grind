Serialization turns a data structure into a flat sequence of characters so that it can be saved to a file or sent somewhere else, and later rebuilt exactly as it was.

Implement a `Codec` class for binary trees with two methods:

- `serialize(root)` takes the root of a binary tree and returns a string.
- `deserialize(data)` takes a string produced by `serialize` and returns the root of the original tree.

The judge calls `deserialize(serialize(root))` (on two separate `Codec` instances) and checks that the rebuilt tree has the same shape and values as the input. The string format is entirely your choice; the bracketed level-order notation used in the examples is only how test trees are written down, and you do not have to use it.

<!-- examples -->

**Constraints:**

- The tree contains between `0` and `10^4` nodes.
- `-1000 <= Node.val <= 1000`
