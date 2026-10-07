A **word ladder** from `beginWord` to `endWord` using a dictionary `wordList` is a sequence of words `beginWord = s1 -> s2 -> ... -> sk = endWord` such that:

- each pair of consecutive words differs in exactly one letter position, and
- every word after the first, `s2` through `sk`, appears in `wordList`. (`beginWord` itself does not need to be in the list.)

Return the **number of words** in the shortest such ladder, or `0` if no ladder exists.

<!-- examples -->

**Constraints:**

- `1 <= beginWord.length <= 10`
- `endWord.length == beginWord.length`
- `1 <= wordList.length <= 5000`
- `wordList[i].length == beginWord.length`
- `beginWord`, `endWord` and every `wordList[i]` consist of lowercase English letters.
- `beginWord != endWord`
- All words in `wordList` are distinct.
