A **trie** (pronounced "try"), also called a prefix tree, stores a set of strings so that looking up whole words and word beginnings is fast. Tries power features such as autocomplete and spell checking.

Implement the `Trie` class:

- `Trie()` creates an empty trie.
- `void insert(String word)` adds `word` to the trie.
- `boolean search(String word)` returns `true` if `word` was inserted earlier, and `false` otherwise.
- `boolean startsWith(String prefix)` returns `true` if at least one previously inserted word begins with `prefix`, and `false` otherwise.

A word counts as starting with itself, so `startsWith` is true for every inserted word.

<!-- examples -->

**Constraints:**

- `1 <= word.length, prefix.length <= 2000`
- `word` and `prefix` contain only lowercase English letters.
- At most `3 * 10^4` calls in total are made to `insert`, `search`, and `startsWith`.
