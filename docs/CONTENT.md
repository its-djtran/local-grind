# Authoring problems

Everything the app shows about a problem is compiled from `content/` into `assets/problems.db`
by `lg-content`. Adding a problem never requires touching Rust or TypeScript.

```
content/
  lib/lg_gen.py            shared helpers for test generators
  lists/*.toml             problem lists (Blind 75, Grind 75, ...)
  companies.json           company tags keyed by slug
  problems/<slug>/
    problem.toml           metadata, signature, judge mode, examples, hints
    description.md         statement (examples are injected at <!-- examples -->)
    editorial.md           approaches, from brute force to optimal, with complexity
    solution.py            reference (optimal) solution - produces every expected output
    solution.{js,cpp,rs}   optional references in other languages (verified, benchmarked)
    gen.py                 test generator - prints one JSON argument array per line
    benchmark.json         optional; written by `lg-content bench`
```

Validate a problem while you work on it:

```sh
cargo run -q -p lg-content -- check <slug>      # build one problem without writing the DB
cargo run -q -p lg-content -- build             # rebuild assets/problems.db
cargo run -q -p lg-content -- bench <slug>      # record reference runtime/memory (10 runs)
cargo run -q -p lg-content -- starter <slug> cpp  # print a language's starter code
```

`check` runs the generator, executes the reference solution through the real judge, verifies
every example's stated output, validates every argument against the signature, and requires at
least 30 tests.

## Writing style and licensing

Content is contributed under the project's MIT OR Apache-2.0 dual license (see `README.md`).

Statements and editorials are **original writing**. Describe the same problem (same function,
same examples, same constraints and edge cases) in your own words; never paste text from
LeetCode or other sites. Keep LeetCode's problem number as `id` and its URL slug as the
directory name so lists and company tags line up.

- `description.md`: the task in a few short paragraphs, then `<!-- examples -->`, then
  `**Constraints:**` as a bullet list, then an optional `**Follow-up:**`. In inline code, `^`
  followed by digits renders as a superscript: `` `1 <= n <= 10^4` ``.
- `editorial.md`: `## Approach N: Name` sections from naive to optimal, each with intuition,
  algorithm, a Python implementation, and `**Time:**` / `**Space:**` complexity. The last
  approach should match `solution.py`.
- Hints (in `problem.toml`): 2-4 progressively stronger nudges; never the full solution.

## problem.toml

```toml
id = 1                      # LeetCode problem number
title = "Two Sum"
difficulty = "Easy"         # Easy | Medium | Hard
topics = ["Array", "Hash Table"]
time_limit_ms = 2000        # optional; budget for the summed solution time over all tests
hints = ["...", "..."]

[signature]                 # see "Signatures" below
kind = "function"
name = "twoSum"
params = [{ name = "nums", type = "int[]" }, { name = "target", type = "int" }]
returns = "int[]"

[judge]                     # optional; default is exact comparison
mode = "unordered"

[[examples]]                # at least one; examples are always the first tests
input = """
[2,7,11,15]
9"""                        # one JSON value per parameter, one per line
output = "[0,1]"
explanation = "optional"
```

### Types

`int`, `long`, `double`, `bool`, `string`, `char` (one-character string), `ListNode`
(serialized `[1,2,3]`), `TreeNode` (level order with `null`: `[1,null,2]`), `Node` (undirected
graph as a 1-indexed adjacency list: `[[2,4],[1,3],[2,4],[1,3]]`), `void`, and lists of any of
these with `[]` suffixes (`int[][]`, `char[][]`, `ListNode[]`, ...).

### Signatures

| kind | Use for | Input lines |
|---|---|---|
| `function` | `class Solution` with one method | one per parameter |
| `design` | a class driven by operations (LRU Cache, Trie) | `["Cls","op",...]` then `[[ctor args],[op args],...]` |
| `codec` | `decode(encode(x))` round trips (Serialize Tree, Encode Strings) | the value to encode |

`function` options:

- `returns = "void"` with `output_param = "nums"` judges an in-place mutation.
- `ref_of = "root"` on a `TreeNode` parameter means the input holds a node *value* to be resolved
  to that node in the `root` tree (Lowest Common Ancestor). Pair with `output_mode = "node_val"`
  to judge a returned node by its value.
- `special = "list_cycle"` with a hidden int parameter (`hidden = true`) links the list tail to
  that index (Linked List Cycle; `-1` for none).
- `special = "bad_version"` with a hidden int parameter provides `isBadVersion(v)` returning
  `v >= bad` (First Bad Version).
- `special = "deep_copy"` rejects outputs that share nodes with the input (Clone Graph).

`design` example:

```toml
[signature]
kind = "design"
class = "MinStack"
constructor = []
methods = [
  { name = "push", params = [{ name = "val", type = "int" }], returns = "void" },
  { name = "getMin", params = [], returns = "int" },
]
```

`codec` example:

```toml
[signature]
kind = "codec"
class = "Codec"
encode = { name = "serialize", params = [{ name = "root", type = "TreeNode" }], returns = "string" }
decode = { name = "deserialize", params = [{ name = "data", type = "string" }], returns = "TreeNode" }
```

### Judge modes

| mode | Meaning |
|---|---|
| `exact` (default) | structural equality (`2 == 2.0`) |
| `unordered` | top-level list in any order (Top K Frequent, Two Sum) |
| `unordered_nested` | outer and inner lists in any order (Group Anagrams, 3Sum, Subsets) |
| `float` + `tolerance = 1e-5` | numeric tolerance (Find Median) |
| `validator` + `name = "..."` | custom check in `crates/core/src/judge.rs` (`alien_order`) |

Pick the loosest mode the problem statement allows ("return in any order") and no looser.

## gen.py

```python
import sys
sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(1)                     # always seed: builds must be reproducible
emit([1, 2], 3)                # one call per test: positional arguments of the solution
```

Aim for 50-100 tests that together catch wrong answers *and* wrong complexity:

1. **Edge cases**: minimum sizes, empty structures where allowed, duplicates, negatives, zeros,
   extreme values at the constraint bounds, all-equal inputs, already-sorted / reverse-sorted.
2. **Small random cases** (dozens) across the value range - these catch logic bugs.
3. **Large cases at the constraint limits** (a handful) - these make `O(n^2)` solutions exceed
   the time limit when the optimal is `O(n log n)` or `O(n)`. Keep total suite data reasonable
   (a few MB at most).
4. Respect every guarantee in the statement (e.g. "exactly one answer", "intervals sorted",
   "graph is connected"): generate, then verify the guarantee and retry if it does not hold.

Helpers in `lg_gen`: `rng`, `emit`, `int_array`, `distinct_ints`, `random_string`,
`tree_level_order`, `bst_level_order`.

## solution.py

The optimal solution in the exact shape users write (`class Solution:` etc.). It runs under the
same harness as user code, so `List`, `Optional`, `ListNode`, `TreeNode`, `Node`, `deque`,
`heapq`, `defaultdict`, `Counter`, `bisect`, `lru_cache`, `math` and friends are pre-imported.
Its output defines the expected answer for every generated test, so it must be correct; when in
doubt, cross-check it against a brute force inside `gen.py` for small inputs.

## Other languages

`solution.js`, `solution.cpp` and `solution.rs` are optional optimal solutions written the way
LeetCode users write them in that language (see the starter code the app generates). They are
not used for expected outputs; `check`/`build` require each to be Accepted on the full suite
(skipped when that toolchain is not installed), and `bench` records a per-language benchmark so
submissions are compared with a reference in the same language. Rust cannot express graph-node
or cyclic-list problems; do not add `solution.rs` there.

`lg-content judge <slug> file.{py,js,cpp,rs}` judges any file in the language given by its
extension.

## Lists and companies

`content/lists/<id>.toml`:

```toml
id = "blind75"
name = "Blind 75"
description = "..."
position = 1
items = [["Array", "two-sum"], ["Array", "best-time-to-buy-and-sell-stock"]]  # [group, slug]
```

Every slug in a list and in `companies.json` must name a problem in `content/problems/`;
`build` fails otherwise, so add a problem before referencing it.

`content/companies.json` maps slugs to `[{company, frequency, recency}]` and is regenerated from
the liquidslr/leetcode-company-wise-problems dataset by `scripts/import_companies.py`.
