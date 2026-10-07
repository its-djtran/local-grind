You are leading a product release that has gone through versions numbered `1, 2, ..., n`. One version introduced a defect, and because each version is built on top of the previous one, that version and **every later version** are bad. The versions before it are all fine.

You can query an API `isBadVersion(version)` that returns whether a given version is bad. Find and return the **first** bad version, calling the API as few times as you can.

In each test, `bad` is the hidden first bad version used by the API; your function receives only `n`.

<!-- examples -->

**Constraints:**

- `1 <= bad <= n <= 2^31 - 1`
