Build a data structure that stores words and can tell whether a stored word matches a pattern, where the pattern may contain wildcards.

Implement the `WordDictionary` class:

- `WordDictionary()` creates an empty dictionary.
- `void addWord(word)` stores `word`.
- `bool search(word)` returns `true` if some stored word matches `word`, and `false` otherwise. The query may contain dots `'.'`; each dot matches exactly one arbitrary letter. A match must cover the whole stored word, so the lengths have to be equal.

<!-- examples -->

**Constraints:**

- `1 <= word.length <= 25`
- In `addWord`, `word` consists of lowercase English letters.
- In `search`, `word` consists of `'.'` or lowercase English letters.
- A `search` query contains at most `2` dots.
- At most `10^4` calls in total are made to `addWord` and `search`.
