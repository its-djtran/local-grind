An alien language is written with the lowercase English letters, but its alphabet orders those letters in some unknown way.

You are handed a list `words` that is claimed to be sorted according to the alien alphabet, using the usual dictionary rule: compare two words at the first position where they differ, and if one word is a prefix of the other, the shorter word comes first.

Reconstruct the alien alphabet: return a string containing **every distinct letter that appears in `words`**, each exactly once, ordered so that `words` really is sorted under that alphabet. If several alphabets work, return any of them. If no alphabet can make the list sorted, return the empty string `""`.

<!-- examples -->

**Constraints:**

- `1 <= words.length <= 100`
- `1 <= words[i].length <= 100`
- `words[i]` consists only of lowercase English letters.
