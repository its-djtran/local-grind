Design a scheme that packs a **list of strings** into a **single string** and unpacks it again, as if one machine encoded the list, sent the resulting string over a network, and a second machine decoded it.

Implement the `Codec` class:

- `encode(strs)` takes a list of strings and returns one string representing all of them.
- `decode(s)` takes a string produced by `encode` and returns the original list, with the same strings in the same order.

The judge calls `encode` on one `Codec` instance and passes its result to `decode` on a **different** instance, so no information can be kept in object state between the two calls. Your encoding may be any string you like, but the decoded list must equal the input exactly.

Strings may contain **any** of the 256 extended ASCII characters (codes `0` to `255`), including spaces, digits, punctuation such as `#`, `,` and `/`, quotes, newlines and other control characters. Do not rely on a built-in serializer (such as `eval`, `json` or `pickle`); work out the format yourself.

<!-- examples -->

**Constraints:**

- `1 <= strs.length <= 200`
- `0 <= strs[i].length <= 200`
- `strs[i]` may contain any of the 256 extended ASCII characters.

**Follow-up:** Can your scheme handle an arbitrary character set, not just these 256 characters?
