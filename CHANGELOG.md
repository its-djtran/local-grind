# Changelog

All notable changes to Local Grind are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html); see "Versioning" in `README.md` for
what counts as a breaking change. Releases are cut with `scripts/bump_version.sh`, which turns
the [Unreleased] section into the new version's entry.

## [Unreleased]

## [0.2.0] - 2026-10-08

### Added
- Versioning: the app follows SemVer with the workspace `Cargo.toml` as the single source of the
  version; `scripts/bump_version.sh` cuts a release (commit and `vX.Y.Z` tag); Settings shows
  the version and the git commit the app was built from; this changelog.
- Releases are built only from a clean, tagged tree (`scripts/release.sh --untagged` allows
  local test builds).

### Changed
- Python solutions require Python 3.9 or newer (3.8 passed the toolchain check but failed
  every run).
- The judge's time budget counts CPU time, so a correct solution is no longer timed out just
  because the machine is busy.
- Faster problem opening, review suggestions, recalibration and Submit; `lg-content judge` is
  about 4x and `lg-content check` about 5x faster.
- Building the catalog fails on unknown slugs in lists or `companies.json` (they were silently
  dropped) and on generators that run longer than 120 s.

### Fixed
- Submitting again after Accepted, or without the timer running, no longer records a fake solve.
- The "Solved" tag appears right after an accepted submission.
- Crashes and timeouts are no longer reported as unreadable output.
- C++ solutions no longer accumulate memory across test cases (false Memory Limit Exceeded).
- One unreadable setting no longer resets all settings, including toolchain paths.
- Heatmap, streak and weekly-trend days use each day's own timezone offset (daylight saving).
- History export and import no longer freeze the window.
- Arrow keys scroll outside the problem list; a theme toggled from the top bar is not undone by
  the Settings page; the shortcuts overlay closes on Escape; the System theme follows the OS.
- Workspace races (pausing or resetting a starting timer, quick problem switching, failed
  language switches) and undo restoring another language's code after a switch.
- The Stats page ignores responses to superseded requests; failed list deletions are reported.
- Compile-cache eviction is least-recently-used as intended; `lg-content bench` cleans up on
  failure; `check.sh --no-coverage` runs the catalog check.

### Removed
- Unused dependencies and commands, dead code, and stray tracked files.

## [0.1.0] - 2026-10-07

First release.

### Added
- Offline LeetCode-style practice: 111 problems (Blind 75 and Grind 75) with original
  statements, editorials, hints, generated test suites and company tags.
- Split workspace with a CodeMirror editor (standard or vim keys), Run against custom testcases
  and Submit against the full suite, with runtime and memory compared to the optimal reference.
- Python, JavaScript, C++ and Rust, with reference solutions and benchmarks in each language;
  Python is bundled, JavaScript falls back to macOS's JavaScriptCore.
- Review tool: solve timer, per-difficulty statistics, recent-vs-overall trends, heatmap,
  streaks, topic and list progress, and a spaced-review queue.
- Personal notes and lists, history export and import, dark and light themes, adjustable text
  size, evaluation settings and local benchmark recalibration.
- macOS `.app` and `.dmg` built by `scripts/release.sh`.
