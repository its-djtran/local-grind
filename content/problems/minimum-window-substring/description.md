You are given two strings `s` and `t` of lengths `m` and `n`. Find the **shortest substring** of `s` that contains every character of `t`, counting repeats: if `t` contains a character three times, the substring must contain it at least three times as well. Return that substring.

If no substring of `s` qualifies, return the empty string `""`.

The tests are built so that whenever an answer exists, it is **unique**.

<!-- examples -->

**Constraints:**

- `m == s.length`
- `n == t.length`
- `1 <= m, n <= 10^5`
- `s` and `t` contain only uppercase and lowercase English letters.

**Follow-up:** Can you find a solution that runs in `O(m + n)` time?
