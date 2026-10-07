## Approach 1: Regular Expression

**Intuition.** The accepted prefix has a fixed shape: optional spaces, an optional single sign,
then digits. A regular expression can capture it directly, and Python integers make clamping easy.

```python
import re


class Solution:
    def myAtoi(self, s: str) -> int:
        m = re.match(r" *([+-]?\d+)", s)
        if not m:
            return 0
        return max(-2**31, min(2**31 - 1, int(m.group(1))))
```

- **Time:** `O(n)`.
- **Space:** `O(n)` for the matched text. Note that converting a 200-digit number is fine in
  Python but would overflow in languages with fixed-width integers.

## Approach 2: Single Scan With Early Clamping (Optimal)

**Intuition.** Walk the string once, stage by stage, building the value digit by digit. To stay
honest about 32-bit limits (as you must in C++ or Java), stop as soon as the value can no longer
fit: before computing `value * 10 + d`, check whether it would exceed `2^31 - 1`. For negative
numbers the limit is `2^31`, which clamps to `-2^31` once the sign is applied, so it is enough to
compare against `2^31 - 1` and return the appropriate bound.

**Algorithm.**

1. Skip spaces with an index `i`.
2. If `s[i]` is `'+'` or `'-'`, record the sign and advance once.
3. While `s[i]` is a digit `d`: if `value > (2^31 - 1 - d) // 10`, the next step would overflow, so
   return `2^31 - 1` for a positive sign and `-2^31` for a negative one. Otherwise set
   `value = value * 10 + d`.
4. Return `sign * value`.

```python
class Solution:
    def myAtoi(self, s: str) -> int:
        INT_MAX, INT_MIN = 2**31 - 1, -2**31
        i, n = 0, len(s)
        while i < n and s[i] == " ":
            i += 1
        sign = 1
        if i < n and s[i] in "+-":
            sign = -1 if s[i] == "-" else 1
            i += 1
        value = 0
        while i < n and s[i].isdigit():
            d = ord(s[i]) - ord("0")
            if value > (INT_MAX - d) // 10:
                return INT_MAX if sign == 1 else INT_MIN
            value = value * 10 + d
            i += 1
        return sign * value
```

- **Time:** `O(n)` — each character is examined at most once.
- **Space:** `O(1)`.
