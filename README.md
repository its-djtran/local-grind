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

## Installing

Build the app with `scripts/release.sh` (see below); it produces
`target/release/bundle/dmg/Local Grind_<version>_<arch>.dmg`. The app is not notarized, so on
first launch macOS asks for confirmation: right-click the app and choose **Open**.

Languages: Python works out of the box (a relocatable CPython is bundled). JavaScript uses
Node.js when installed and otherwise macOS's built-in JavaScriptCore. C++ needs the Xcode
Command Line Tools (`xcode-select --install`) and Rust needs rustup. Settings → Evaluation shows
what was found and lets you point at a specific toolchain.

## Development

Requirements: Rust (stable), Node 20+, Python 3 (used to run Python solutions and to generate
tests).

```sh
cargo run -q -p lg-content -- build   # compile content/ into assets/problems.db
cd app && npm install && npx tauri dev
```

`scripts/release.sh` builds the distributable app (it downloads the bundled Python, builds the
catalog, runs every check, builds the .app and .dmg, and smoke-tests the built app).

See `CLAUDE.md` for all commands, `docs/ARCHITECTURE.md` for how the pieces fit together,
`docs/CONTENT.md` to add problems, and `docs/PLAN.md` for the roadmap.

## License

Local Grind is dual-licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option. This covers the code and the problem content (statements, editorials, tests and
reference solutions are original writing describing well-known interview problems). Bundled
third-party components keep their own licenses; see [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
Local Grind by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
