# Local Grind architecture

Local Grind is an offline LeetCode-style practice and review app. A Rust core does all the
work; a Tauri shell exposes it to a Svelte UI; problem content is compiled ahead of time into a
read-only SQLite catalog.

```
            build time                                         run time
┌──────────────────────────────┐            ┌──────────────────────────────────────────┐
│ content/                     │            │ Local Grind.app                          │
│  problems/<slug>/*.toml,.md, │ lg-content │  ┌─────────────┐  invoke   ┌───────────┐  │
│  solution.py, gen.py         ├──────────▶ │  │ Svelte UI   │ ────────▶ │ Tauri     │  │
│  lists/*.toml                │  build     │  │ (WKWebView) │ ◀──────── │ commands  │  │
│  companies.json              │            │  └─────────────┘  events   └─────┬─────┘  │
└──────────────────────────────┘            │                                  │        │
              │                             │                      localgrind-core      │
              ▼                             │   ┌──────────┐ ┌────────┐ ┌──────────────┐ │
      assets/problems.db ──── bundled ────▶ │   │ Catalog  │ │ UserDb │ │ Runner/Judge │ │
                                            │   │ (r/o)    │ │ (r/w)  │ │ + drivers    │ │
                                            │   └──────────┘ └────────┘ └──────┬───────┘ │
                                            └──────────────────────────────────┼────────┘
                                                                               ▼
                                                            python3 lg_harness.py input.jsonl
```

## Repository layout

| Path | What lives there |
|---|---|
| `crates/core` | `localgrind-core`: all application logic (no GUI dependencies) |
| `crates/core/drivers/<lang>/` | Per-language harness sources embedded into the binary |
| `crates/content` | `lg-content`: builds, checks, judges and benchmarks problem content |
| `app/src-tauri` | Tauri shell: resolves file locations and maps commands onto `Service` |
| `app/src` | Svelte 5 UI (`lib/` logic, `components/` views, `mock/` in-memory backend) |
| `content/` | Problem sources, lists, company tags — see `docs/CONTENT.md` |
| `assets/` | Build outputs (`problems.db`, gitignored) and branding sources |
| `scripts/` | `check.sh` (full gate), `import_companies.py`, `webshot.swift` |

## Core modules (`crates/core/src`)

| Module | Responsibility |
|---|---|
| `types` | Difficulty, `ValueType` (`int[][]`, `TreeNode`, ...), `Signature` (function / design / codec, with specials such as hidden params and node references), `Comparator` |
| `lang` | `Language`, `Driver` trait (starter code, runnable `Program`, `supports`), toolchain detection and probing; drivers in `lang/{python,javascript,cpp,rust}.rs` |
| `runner` | Spawns a `Program` in its own process group, streams results, enforces time limits, measures peak RSS via `wait4`, produces a `RunReport` with a `Verdict` |
| `judge` | Output comparison modes and problem-specific validators |
| `catalog` | Read API over `problems.db` and `CatalogWriter` used by `lg-content` |
| `userdb` | Migrations and CRUD for submissions, drafts, solves (timers) and settings |
| `metrics` | One-pass statistics and the review queue |
| `portable` | JSON export/import of all user data |
| `settings` | User preferences with sanitizing |
| `service` | The facade the shell calls: bootstrap, problem views, run, submit, stats, ... |
| `timeutil` | ISO-8601 helpers without a date crate |

### Judge protocol

Every language driver produces a program that is run as `<cmd> input.jsonl`, where each input
line is a JSON array of arguments. After each test the harness flushes user stdout and writes
`\x1eLG {"i":N,"ns":T,"out":<json>}`; an exception yields `{"i":N,"error":"..."}` and a source
error in an interpreted language yields `{"compile_error":"..."}`. Anything else on stdout is the
user's output, attributed to the following test.

The runner judges each line as it arrives (so a submission stops at the first wrong answer),
sums the harness-measured solution time (interpreter start-up and parsing are excluded) against
the problem's time limit, keeps a wall-clock watchdog for hangs, and kills the whole process
group when done. The watchdog also samples the child's resident memory every 20 ms
(`proc_pidinfo` on macOS, `/proc` on Linux) and kills it past the memory limit. Peak memory
comes from `wait4`'s `ru_maxrss`. With low-priority judging enabled the child lowers its own
priority (nice 10) before exec.

The runner raises each child's stack soft limit to its hard limit before exec, so
single-threaded runtimes (JavaScriptCore) can recurse deeply too. `LOCALGRIND_TOOLCHAIN_<LANG>`
overrides toolchain detection (used to test the JavaScriptCore path).

`lang::probe` checks each toolchain (`python3 --version`, minimum 3.8); the UI shows the
result in Settings and a banner when nothing usable is installed. GUI apps launched from Finder
get a minimal `PATH`, so detection also searches Homebrew and other common prefixes.

Compiled languages list their build outputs as `Program::artifacts`; the runner caches them
under the app cache directory keyed by a hash of the sources and compile command, so Run
followed by Submit compiles once.

All comparison happens in Rust, so adding a language means writing a harness that decodes
arguments and encodes results — never reimplementing judging.

### Language drivers

| Language | Toolchain | How user code runs |
|---|---|---|
| Python | `python3` ≥ 3.8 | `lg_harness.py` execs `solution.py` with a LeetCode-style prelude, on a thread with a 512 MB stack |
| JavaScript | `node` ≥ 16, else macOS's `jsc` | `lg_harness.js` runs `solution.js` via `vm` inside a Worker with a 256 MB stack (Node), or via `load()` on a deep main-thread stack (JavaScriptCore); synchronous stdout |
| C++ | `clang++` (C++20, `-O2`) | `solution.cpp` = prelude + `#line 1` user code + driver + generated `lg_case.inc`; runs on a 512 MB-stack thread |
| Rust | `rustc` (edition 2021, `-O`, overflow checks) | `lg_main.rs` `include!`s the user's `solution.rs` and generated `lg_case.rs` into one module |

Dynamic languages decode arguments from the signature at runtime; typed languages get a
generated per-signature `lg_case` that decodes into native types. Drivers report which
signatures they can express (`Driver::supports`); Rust omits graph-node and cyclic-list
problems, matching LeetCode. Problems ship reference solutions per language; Run uses the
reference in the user's language (falling back to Python) to compute expected outputs for
custom testcases, and benchmarks are recorded per language.

### Run vs Submit

- **Run** executes the user's code on the console testcases and, in parallel (when
  `max_parallel_jobs` > 1), a bundled reference solution on the same inputs to obtain expected
  outputs, so custom inputs work.
- **Submit** judges against the full stored suite, records a `Submission`, and applies it to the
  problem's active `Solve`: failures increment its error count; Accepted completes it with the
  elapsed time from the UI timer.

## Data

**`problems.db` (read-only, bundled).** Tables: `problems` (metadata, signature JSON,
pre-rendered description/editorial HTML, examples, hints), `tests` (deflate-compressed JSONL per
problem), `solutions` (code plus build-time highlighted HTML per language), `benchmarks`, `lists`/`list_items`, `problem_companies`, `meta`. The app
loads summaries, lists and the company index once at startup; problem detail and test suites
are read on demand.

**`user.db` (app data dir).** `submissions`, `drafts`, `solves` (at most one active per problem,
enforced by a partial unique index), `settings` (one JSON row), `local_benchmarks` (recalibration results), `notes` (one per problem). Schema changes are append-only
entries in `userdb::MIGRATIONS`, tracked with `PRAGMA user_version`.

## Desktop shell (`app/src-tauri`)

`resolve_catalog` finds `problems.db` (env override → bundled resource → repository build), the
user database goes in the platform app-data directory, and each `#[tauri::command]` forwards to
`Service`. Long-running commands (run, submit, stats) execute on a blocking thread; submission
progress is pushed to the UI as `submit-progress` events.

## UI (`app/src`)

| Path | Role |
|---|---|
| `lib/api.ts` | Typed wrappers over `invoke` and events |
| `lib/types.ts` | TypeScript mirrors of the Rust payloads |
| `lib/state.svelte.ts` | App-wide state (bootstrap data, settings, theme application, toasts) |
| `lib/router.svelte.ts` | Hash router: `#/problems`, `#/problems/<id>`, `#/stats`, `#/settings` |
| `lib/workspace.svelte.ts` | Problem workspace controller: drafts, timer, run, submit, history |
| `lib/filter.ts`, `testcases.ts`, `timer.ts`, `format.ts`, `calendar.ts`, `review.ts`, `editor.ts` | Pure, unit-tested logic |
| `components/` | Views; kept thin over the `lib/` logic |
| `mock/backend.ts` | In-memory backend used by component tests and `npm run dev:mock` |

Theming uses CSS custom properties on `<html data-theme>`; font family and sizes are variables
driven by settings. Inconsolata is bundled via `@fontsource` (no network access needed).

## Testing

- Rust unit tests live beside the code; `crates/core/tests/python_e2e.rs` runs real Python
  solutions through the harness; `lg-content` tests build fixture content end to end.
- `crates/core/tests/catalog_e2e.rs` (ignored by default; run by `scripts/check.sh` after a
  fresh `lg-content build`) submits every problem's reference solution through `Service` and
  requires Accepted, so harness changes cannot silently break content.
- Frontend tests run under Vitest + jsdom. Components are tested against the mock backend via
  Tauri's `mockIPC`, so the UI is exercised through the same command interface as production.
- `scripts/check.sh` is the gate: formatting, clippy, svelte-check, all tests, and a 90%
  coverage floor for both halves.
- `scripts/smoke.sh` is the end-to-end test of the real app: it builds the desktop binary and
  launches it with `LOCALGRIND_SMOKE=1` and a throwaway `LOCALGRIND_DATA_DIR`; `lib/smoke.ts`
  drives the real UI through the core flows and the app exits with the result.
- Visual checks: `npm run dev:mock` serves the UI against the mock backend; `scripts/webshot.swift`
  renders a URL with WebKit (the same engine Tauri uses on macOS) to a PNG.
