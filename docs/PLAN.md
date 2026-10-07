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
- [ ] All 111 problems with statement, editorial, hints, reference solution, generator
      (≥ 30 tests, aim 50–100, large cases that reject wrong complexity) — 1 / 111 done
- [ ] Reference benchmarks (avg of 10 runs) for every problem

### M4 — Additional languages
- [ ] Driver abstraction validated with a compiled language (C++ first: clang ships with Xcode CLT)
- [ ] JavaScript, Java, Rust, Go drivers; toolchain detection and compile caching
- [ ] Per-language reference solutions and benchmarks where available

### M5 — Review tool
Pulled forward while building M1 (done):
- [x] Solve timer (auto-start on first edit, pause/resume, persisted, stops on Accepted)
- [x] Metrics engine: per-difficulty min/avg/max time and error rate, recent-N vs overall,
      submission heatmap, topic breakdown, streaks, list progress
- [x] Review queue (45% pass rate, 35% relative slowness, 20% staleness) with reasons
- [x] Stats page

Remaining:
- [ ] Trends over time (solve time and error rate per week)
- [ ] "Practice next" suggestions on the problem list (review-due and unsolved-in-list)
- [ ] Performance check: stats over 10k synthetic solves stays fast

### M6 — Portability & polish
- [x] Export / import complete user history (JSON)
- [x] Search (title / number / topic) and filters (list, difficulty, status, topic, company)
- [~] Keyboard shortcuts: run (⌘') and submit (⌘⏎) done; navigation shortcuts pending
- [ ] Local benchmark recalibration (uses `max_parallel_jobs`)
- [ ] Lazy-load vim keymap and other heavy editor extensions

### M7 — Distribution
- [x] App icon
- [ ] `.dmg` packaging and a release script
- [ ] Startup and navigation performance budget checks

### M8+ — Refinement
- Additional problem sets, UI polish driven by usage.
