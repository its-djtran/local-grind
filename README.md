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

### Updating

Quit Local Grind, open the new `.dmg` and drag the app onto **Applications**, choosing
**Replace**. Your history, settings, notes and lists live in
`~/Library/Application Support/dev.localgrind.app/`, not in the app, so they carry over; a newer
version upgrades them automatically. Downgrading is not supported (an older version refuses to
open data written by a newer one), so export your history from Settings before upgrading if you
may want to go back.

The running version and the commit it was built from are shown at the bottom of Settings (and
in Finder's Get Info); changes between versions are listed in [CHANGELOG.md](CHANGELOG.md).

## Versioning

Local Grind follows [Semantic Versioning](https://semver.org/). Its public interface is what
users and contributors depend on: the user data and history export format, the content format
in `content/` together with the `lg-content` commands, and the app's documented behaviour.

- **MAJOR**: a version that cannot read user data or exports written by an earlier version, or
  that requires existing problem content to be rewritten.
- **MINOR**: new features, problems, lists or languages, and backward-compatible changes.
- **PATCH**: bug fixes and content corrections.

While the version is 0.y.z, a minor release may also include breaking changes; they are called
out in the changelog. The version lives in one place, `[workspace.package]` in `Cargo.toml`;
`scripts/bump_version.sh major|minor|patch` cuts a release (the changelog entry, a release
commit and a `vX.Y.Z` tag) and `scripts/release.sh` builds only tagged releases.

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
