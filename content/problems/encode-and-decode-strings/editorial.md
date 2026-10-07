## Approach 1: Separator With Escaping

**Intuition.** A plain separator fails because the separator character can occur inside a string.
Escaping fixes that: inside the data, replace every `/` with `//`, then end each string with
`/:`. While decoding, a `/` followed by another `/` is a literal slash, and a `/` followed by `:`
ends the current string. Since every literal `/` is doubled, the decoder can never mistake data
for a terminator.

**Algorithm.**

1. Encode: for each string, append `s.replace("/", "//") + "/:"`.
2. Decode: scan left to right. On `/`, look at the next character: `/` adds a literal slash,
   `:` finishes the current string. Any other character is copied as is.

```python
class Codec:
    def encode(self, strs: List[str]) -> str:
        return "".join(s.replace("/", "//") + "/:" for s in strs)

    def decode(self, s: str) -> List[str]:
        out, cur, i = [], [], 0
        while i < len(s):
            if s[i] == "/":
                if s[i + 1] == ":":
                    out.append("".join(cur))
                    cur = []
                else:
                    cur.append("/")
                i += 2
            else:
                cur.append(s[i])
                i += 1
        return out
```

- **Time:** `O(N)` for both directions, where `N` is the total number of characters.
- **Space:** `O(N)` for the output; the encoding is at most about twice the input size.

## Approach 2: Length Prefixes (Optimal)

**Intuition.** If the decoder knows how long the next string is, it never has to inspect the
string's characters at all, so they can be anything. Write each string as its length in decimal,
a `#` to mark where the number ends, and then the raw characters. The decoder reads digits up to
the first `#` (the number itself contains only digits, so that `#` is always the marker), then
slices out exactly that many characters, even if they include `#` or digits.

**Algorithm.**

1. Encode: concatenate `f"{len(s)}#{s}"` for every string.
2. Decode: with a cursor `i`, find the next `#` at `j`, read `n = int(s[i:j])`, take
   `s[j + 1 : j + 1 + n]`, and move `i` to `j + 1 + n`. Repeat until the end.

An empty string becomes `"0#"`, so `[""]` and `[]` encode differently and both round-trip
correctly. Nothing depends on the character set, which answers the follow-up.

```python
class Codec:
    def encode(self, strs: List[str]) -> str:
        return "".join(f"{len(s)}#{s}" for s in strs)

    def decode(self, s: str) -> List[str]:
        out, i = [], 0
        while i < len(s):
            j = s.index("#", i)
            n = int(s[i:j])
            out.append(s[j + 1:j + 1 + n])
            i = j + 1 + n
        return out
```

- **Time:** `O(N)` for both directions.
- **Space:** `O(N)` for the encoded string and the output list.
