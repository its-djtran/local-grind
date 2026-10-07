## Approach 1: List of Words (Brute Force)

**Intuition.** Keep every inserted word in a set. `search` is then a set lookup, and `startsWith`
checks every stored word for the prefix.

```python
class Trie:
    def __init__(self):
        self.words = set()

    def insert(self, word: str) -> None:
        self.words.add(word)

    def search(self, word: str) -> bool:
        return word in self.words

    def startsWith(self, prefix: str) -> bool:
        return any(w.startswith(prefix) for w in self.words)
```

- **Time:** `insert` and `search` take `O(L)` for a string of length `L`, but `startsWith` takes
  `O(N * L)` when `N` words are stored. With tens of thousands of words and queries this is
  hundreds of millions of comparisons.
- **Space:** `O(total characters inserted)`.

A variation stores every prefix of every word in a second set, making `startsWith` a lookup, but
then inserting a word of length `L` costs `O(L^2)` time and memory, which explodes for words of
length 2000.

## Approach 2: Trie of Nested Maps (Optimal)

**Intuition.** Words that share a beginning should share the work of storing it. Arrange the
characters in a tree: the root represents the empty prefix and each child edge appends one letter,
so every node corresponds to exactly one prefix of some inserted word. A flag on a node records
that a complete word ends there.

**Algorithm.**

- `insert(word)`: starting at the root, follow the child for each letter, creating it if it does
  not exist yet. Mark the final node as the end of a word.
- `startsWith(prefix)`: follow the letters of `prefix`; if a child is missing, no stored word has
  this prefix. If the walk finishes, return `true`.
- `search(word)`: walk the same way, then return whether the final node carries the end flag.
  Without that flag, `search("app")` would wrongly succeed after inserting only `"apple"`.

Each node is a dictionary from letter to child; the special key `"$"` (not a lowercase letter)
serves as the end-of-word flag.

```python
class Trie:
    def __init__(self):
        self.root = {}

    def insert(self, word: str) -> None:
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = True

    def _walk(self, s):
        node = self.root
        for ch in s:
            node = node.get(ch)
            if node is None:
                return None
        return node

    def search(self, word: str) -> bool:
        node = self._walk(word)
        return node is not None and "$" in node

    def startsWith(self, prefix: str) -> bool:
        return self._walk(prefix) is not None
```

- **Time:** `O(L)` per operation, where `L` is the length of the argument, regardless of how many
  words are stored.
- **Space:** `O(total characters inserted)` in the worst case (no shared prefixes); shared
  prefixes are stored once.
