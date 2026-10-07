## Approach 1: Compare Against Every Word (Brute Force)

**Intuition.** Keep a list of the stored words. For a query, check each word of the same length
character by character, letting a dot match anything.

```python
class WordDictionary:
    def __init__(self):
        self.words = []

    def addWord(self, word: str) -> None:
        self.words.append(word)

    def search(self, word: str) -> bool:
        for w in self.words:
            if len(w) == len(word) and all(c == "." or c == x for c, x in zip(word, w)):
                return True
        return False
```

- **Time:** `O(N * L)` per `search` with `N` stored words of length up to `L`; `O(1)` per
  `addWord`. Thousands of queries against thousands of words means tens of millions of
  comparisons.
- **Space:** `O(N * L)`.

## Approach 2: Trie with Wildcard Branching (Optimal)

**Intuition.** Store the words in a trie, where each node represents a prefix and a flag marks the
nodes at which words end. A query with only letters is an ordinary trie lookup. A dot simply means
"any child": instead of following one edge we follow all of them. Because a query has at most two
dots, the search can fan out to at most `26 * 26` nodes, independent of how many words are stored.

**Algorithm.**

- `addWord`: walk the trie, creating missing children, and set the end flag on the last node.
- `search`: keep the set of trie nodes that match the query so far, starting with just the root.
  For each query character, replace every node by its child for that letter, or by all of its
  children for a dot. If the set becomes empty, return `false`. At the end, return whether any
  remaining node has the end flag.

Each node is a dictionary from letter to child; the key `"$"` marks the end of a word and is
skipped when expanding a dot.

```python
class WordDictionary:
    def __init__(self):
        self.root = {}

    def addWord(self, word: str) -> None:
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = True

    def search(self, word: str) -> bool:
        level = [self.root]
        for ch in word:
            if ch == ".":
                level = [child for node in level for key, child in node.items() if key != "$"]
            else:
                level = [node[ch] for node in level if ch in node]
            if not level:
                return False
        return any("$" in node for node in level)
```

A recursive depth-first search that tries each child at a dot works equally well; the
breadth-first version above just avoids recursion.

- **Time:** `O(L)` for `addWord`. For `search`, `O(L)` without dots and `O(26^d * L)` with `d`
  dots, so at most `O(676 * L)` here.
- **Space:** `O(total characters added)` for the trie.
