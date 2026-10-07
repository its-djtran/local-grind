You are given a string `s` made up only of the bracket characters `'('`, `')'`, `'['`, `']'`, `'{'` and `'}'`. Decide whether the brackets are **balanced**.

A string is balanced when:

- every closing bracket is matched by an opening bracket of the same kind,
- each pair is closed in the reverse order it was opened, so pairs nest inside one another rather than overlapping, and
- no opening bracket is left without a partner.

Return `true` if `s` is balanced and `false` otherwise.

<!-- examples -->

**Constraints:**

- `1 <= s.length <= 10^4`
- `s` contains only the characters `'()[]{}'`.
