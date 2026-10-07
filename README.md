# Local Grind

An offline, open-source LeetCode-style practice app with a built-in review tool.

- LeetCode-like split workspace: statement, editorial and submission history on the left; a
  CodeMirror editor and testcase console on the right.
- Run against custom testcases (expected outputs come from a bundled reference solution) or
  submit against a full generated test suite. Every submission records runtime and peak memory,
  shown next to the optimal solution's benchmark.
- A solve timer and review statistics: per-difficulty times and error rates, recent-vs-overall
  trends, an activity heatmap, topic breakdowns and a spaced-review queue.
- Blind 75 and Grind 75 lists, company tags, search and filters.
- Everything is local: problem content ships as a pre-built SQLite catalog, and your history can
  be exported to and imported from a single JSON file.

## Development

Requirements: Rust (stable), Node 20+, Python 3 (used to run Python solutions and to generate
tests).

```sh
cargo run -q -p lg-content -- build   # compile content/ into assets/problems.db
cd app && npm install && npx tauri dev
```

See `CLAUDE.md` for all commands, `docs/ARCHITECTURE.md` for how the pieces fit together,
`docs/CONTENT.md` to add problems, and `docs/PLAN.md` for the roadmap.

Problem statements and editorials in this repository are original writing describing
well-known interview problems. Company tags are derived from
[liquidslr/leetcode-company-wise-problems](https://github.com/liquidslr/leetcode-company-wise-problems).
