On an old phone keypad, each digit from `2` to `9` is printed with a few letters:

| Digit | Letters | Digit | Letters |
|---|---|---|---|
| `2` | `abc` | `6` | `mno` |
| `3` | `def` | `7` | `pqrs` |
| `4` | `ghi` | `8` | `tuv` |
| `5` | `jkl` | `9` | `wxyz` |

The digit `1` has no letters.

Given a string `digits` containing only the digits `2` through `9`, return every string that can be formed by replacing each digit with one of its letters. The strings may be returned in any order. If `digits` is empty, return an empty list.

<!-- examples -->

**Constraints:**

- `0 <= digits.length <= 4`
- `digits[i]` is a digit in the range `['2', '9']`.
