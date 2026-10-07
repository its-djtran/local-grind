# Local Grind — Development Plan

Agent-maintained plan derived from `REQS.md` (the human-authored source of truth).
`REQS.md` must never be edited by agents; this file is where planning lives.

Status legend: `[x]` done · `[~]` partial · `[ ]` not started.
Work is done milestone by milestone. A milestone is closed only when every item is checked (or
explicitly moved, with a reason, to a later milestone) and `scripts/check.sh` passes.

## Technology decisions

| Concern | Choice | Why |
|---|---|---|
| Core logic | Rust crate `localgrind-core` | Fast, testable without a GUI, 90%+ coverage achievable. |
| Desktop shell | Tauri v2 | Rust backend, system WKWebView (no bundled Chromium), fast startup, native `.dmg` bundling. |
| UI | Svelte 5 + TypeScript + Vite | Compiles away the framework; tiny runtime; component tests with Vitest. |
| Editor | CodeMirror 6 | Modular, light, language packs, vim keymap available. |
| Static content | SQLite `problems.db` built by `lg-content` | Pre-rendered HTML, compressed test suites, benchmarks, lists, company tags. |
| User data | SQLite `user.db` in the app data dir | Submissions, drafts, timers, settings; exported/imported as one JSON document. |

Pure-Rust GUI toolkits (egui/iced/slint) were rejected: none ship an editor or rich text
rendering comparable to LeetCode's, which is the core of the "same experience" requirement.

## Decisions log

- **Original problem text.** Statements and editorials are written for Local Grind (same problem,
  signature, examples and constraints) rather than copied, so the repository can be
  redistributed as open source. See `docs/CONTENT.md`.
- **Expected outputs are generated.** Each problem's reference solution, run through the app's
  own judge, produces every expected output; examples are cross-checked at build time.
- **Benchmarks.** The REQS asks for the optimal solution's time/memory (average of 10 runs).
  These are pre-built per language by `lg-content bench`. Because absolute numbers depend on
  the machine, M6 adds an optional local recalibration.
- **YALT is inspiration only** (per user, 2026-10-07): its metrics shaped the review tool, but
  there is no data-model or import compatibility requirement. The YALT importer was removed.
- **Complexity is enforced only where constraints allow it** (2026-10-07). Some problems have
  limits so small (e.g. Same Tree, n ≤ 100) that slower approaches cannot be told apart; LeetCode
  behaves the same. A few limits are tight by necessity: Binary Search (4 ms of solution time,
  reference 0.1 ms) is the only way to reject a linear scan, and Graph Valid Tree (500 ms) and
  Connected Components (300 ms) leave ~40x headroom. Users on slow machines can raise the time
  limit multiplier.
- **Known judge limits.** Codec problems use separate instances for encode/decode, but state
  stored on the class itself is not prevented. "Do not use library sort" style rules (Sort
  Colors) and API-call budgets (First Bad Version) are stated, not enforced.
- **Toolchains at distribution** (2026-10-07). Python is bundled with the app (it is also the
  fallback reference for custom testcases). Compilers are not bundled: C++ and Rust are detected
  (Homebrew, rustup and system paths, since Finder-launched apps get a minimal `PATH`) with
  install guidance and per-language path overrides; JavaScript may use the OS's JavaScriptCore.
- **Reorder: content before additional languages** (2026-10-07). Expected outputs are
  language-neutral, so problem content does not depend on which languages exist, and REQS sets
  Blind 75 ∪ Grind 75 as the minimum bar. Additional languages therefore moved from M2 to M4.

## Milestones

### M0 — Foundations
- [x] Cargo workspace (`crates/core`, `crates/content`, `app/src-tauri`) and frontend in `app/`
- [x] `scripts/check.sh`: rustfmt, clippy `-D warnings`, svelte-check, all tests, 90% coverage
      floor for Rust (tarpaulin) and frontend (vitest)
- [x] `docs/CONTENT.md` authoring guide
- [x] `docs/ARCHITECTURE.md`, `CLAUDE.md` with dev commands, `README.md`

### M1 — Tracer bullet (most of the interface, little content)
- [x] Problem model and signature type system
- [x] Content builder → `problems.db` (markdown pre-rendered, suites compressed)
- [x] Python runner: generated harness, streamed results, verdicts, time + peak memory
- [x] Judge comparators (exact, unordered, unordered-nested, float, validators)
- [x] User DB: submissions, drafts, settings, solve timers
- [x] Tauri shell + commands
- [x] UI: problem list, split problem view (description / editorial / submissions / result),
      editor, testcase console, run + submit, theme toggle, font sizes, settings page
- [x] Three sample problems (Two Sum, Reverse Linked List, Maximum Depth of Binary Tree)
- [x] End-to-end smoke test of the real app: `scripts/smoke.sh` launches the built app in smoke
      mode; the UI drives list → workspace → run → submit → stats → settings against the real
      backend and Python judge, then exits with pass/fail

### M2 — Judge systems
Pulled forward while building M1 (done):
- [x] Value types: ListNode, TreeNode, graph Node, char grids, nested lists
- [x] Design-class problems, codec round trips, hidden-parameter specials (cycle lists, first
      bad version), tree node references (LCA), deep-copy enforcement
- [x] Validators for multi-answer problems (`alien_order`)
- [x] Early stop on first failure; TLE / RE / CE / MLE verdicts
- [x] Author tooling: `lg-content check` and `lg-content judge`

Remaining:
- [x] Evaluation settings: time-limit multiplier, memory limit, interpreter path, max parallel
      judge processes (Run executes user + reference side by side only when > 1), low-priority
      judging (nice 10)
- [x] Live memory-limit enforcement (watchdog samples RSS every 20 ms)
- [x] Toolchain probing (`python3 --version`, minimum 3.8) shown in Settings, plus an app-wide
      banner with install guidance when no toolchain is usable
- Moved to M4: additional languages (see decisions log)

### M3 — Content: Blind 75 ∪ Grind 75 (111 problems)
- [x] Lists: Blind 75 (by topic), Grind 75 (by week)
- [x] Company tags (recency + frequency) from liquidslr/leetcode-company-wise-problems
- [x] All 111 problems with statement, editorial, hints, reference solution, generator
      (8,025 tests total). Each was verified with `lg-content check`; brute-force variants were
      judged to hit the time limit where the constraints make that possible, and plausible
      buggy variants were judged to be rejected
- [x] Reproducible builds (generators run with `PYTHONHASHSEED=0`; two set-order bugs fixed)
- [x] Reference benchmarks (avg of 10 runs) for every problem
- [x] `catalog_e2e` test (in `scripts/check.sh`): every reference solution is submitted through
      the real `Service` against the built catalog and must be Accepted

### M4 — Additional languages
- [x] Multi-language plumbing: per-language toolchain detection, probing and path settings;
      compile cache for compiled languages
- [x] JavaScript driver (LeetCode-style `var f = function` / prototype design / codec
      functions / `solution(isBadVersion)`), Worker with a 256 MB stack, e2e tests, smoke step
- [x] C++ driver (clang++, generated typed harness, 512 MB stack, readable crashes)
- [x] Rust driver (LeetCode conventions; graph-node and cyclic-list problems not offered)
- [x] Per-problem language support (`Driver::supports`), shown in the language picker
- [x] JavaScript, C++ and Rust reference solutions for every problem they can express,
      verified on the full suites; `catalog_e2e` judges all of them
- [x] Run computes expected outputs with the reference in the user's language (falls back to
      Python), so non-Python users do not need Python for custom testcases
- [x] Per-language benchmarks for every reference (442 benchmarks, 10 runs each)
- Deferred (per user, 2026-10-07): Java and Go drivers
- [ ] Per-language reference solutions and benchmarks where available

### M5 — Review tool
Pulled forward while building M1 (done):
- [x] Solve timer (auto-start on first edit, pause/resume, persisted, stops on Accepted)
- [x] Metrics engine: per-difficulty min/avg/max time and error rate, recent-N vs overall,
      submission heatmap, topic breakdown, streaks, list progress
- [x] Review queue (45% pass rate, 35% relative slowness, 20% staleness) with reasons
- [x] Stats page

Remaining:
- [x] Trends over time: 26 weeks of average solve time and error rate as separate line charts
      (crosshair + keyboard readout + table view; gaps for weeks without solves)
- [x] "Practice next" on the problem list: next unsolved problem per list and the top
      review-due problems
- [x] Performance: stats over 20k submissions / 10k solves in ~13 ms (release), pinned by
      `tests/stats_perf.rs`; statistics read a lightweight activity view

### M6 — Portability & polish
- [x] Export / import complete user history (JSON)
- [x] Search (title / number / topic) and filters (list, difficulty, status, topic, company)
- [~] Keyboard shortcuts: run (⌘') and submit (⌘⏎) done; navigation shortcuts and a `?`
      help overlay pending
- [x] Local benchmark recalibration: Settings re-runs every reference solution on this machine
      (3 runs each, up to `max_parallel_jobs` at once, with progress and Stop), stores results in
      `user.db`, and prefers them over bundled numbers (labelled "on this machine")
- [x] Reference solution viewer per language (Editorial tab, defaults to the current language)
- [x] Editorial code blocks and reference solutions syntax-highlighted at build time with
      syntect (class-based static HTML themed by CSS; catalog schema v2)
- [ ] Lazy-load vim keymap and language packs to shrink the initial bundle

### M7 — Distribution
- [x] App icon
- [ ] Bundle a relocatable CPython (python-build-standalone) in the app so Python solutions and
      the Python fallback reference work without any installation
- [ ] Evaluate a zero-install JavaScript path via macOS's built-in JavaScriptCore (`jsc`)
- [ ] First-run toolchain guidance: per-language status with one-line install instructions
      (C++ via `xcode-select --install`, Rust via rustup, Node via installer/Homebrew)
- [ ] `.dmg` packaging and a release script
- [ ] Startup and navigation performance budget checks

### M8+ — Refinement
- Additional problem sets, UI polish driven by usage.
