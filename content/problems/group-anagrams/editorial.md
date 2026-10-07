## Approach 1: Compare Against Every Group

**Intuition.** Process the words one at a time. For each word, look through the groups built
so far and check whether the word is an anagram of the group's first member (by comparing
sorted letters). Join that group, or start a new one.

```python
class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups, keys = [], []
        for word in strs:
            key = sorted(word)
            for i, k in enumerate(keys):
                if k == key:
                    groups[i].append(word)
                    break
            else:
                keys.append(key)
                groups.append([word])
        return groups
```

- **Time:** `O(n * g * k)` where `n` is the number of words, `g` the number of groups and `k`
  the word length. When most words are unrelated, `g` is close to `n` and this is quadratic.
- **Space:** `O(nk)`.

## Approach 2: Hash Map Keyed by Sorted Letters

**Intuition.** Instead of searching for a word's group, compute the group's *name* directly
from the word. Sorting the letters gives the same string for all anagrams and different strings
for non-anagrams, so it is a perfect key for a hash map.

**Algorithm.**

1. Create a map from key to list of words.
2. For each word, append it to `groups["".join(sorted(word))]`.
3. Return the map's values.

```python
class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for word in strs:
            groups["".join(sorted(word))].append(word)
        return list(groups.values())
```

- **Time:** `O(n * k log k)`.
- **Space:** `O(nk)` for the map.

## Approach 3: Hash Map Keyed by Letter Counts (Optimal)

**Intuition.** A word's anagram class is determined by how many of each letter it contains, so
a tuple of 26 counts is also a valid key. Building it is linear in the word length.

```python
class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for word in strs:
            count = [0] * 26
            for ch in word:
                count[ord(ch) - 97] += 1
            groups[tuple(count)].append(word)
        return list(groups.values())
```

- **Time:** `O(n * (k + 26))` — linear in the total input size, versus `O(n * k log k)` for
  sorting. (For short words in Python, the C-implemented `sorted` is about as fast in practice.)
- **Space:** `O(nk)`.
