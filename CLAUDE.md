# Local Grind — notes for agents

`REQS.md` is human-authored and must not be edited. Plans live in `docs/PLAN.md`; keep its
checkboxes truthful and work milestone by milestone. Architecture is in `docs/ARCHITECTURE.md`
(update it when systems change); content authoring is in `docs/CONTENT.md`.

## Commands

Rust tooling lives in `~/.cargo/bin` (add it to `PATH`).

```sh
scripts/check.sh [--no-coverage]                  # full gate: fmt, clippy, svelte-check, tests, catalog, coverage >= 90%
cargo test --workspace                            # Rust tests
cargo run -q -p lg-content -- build               # content/ -> assets/problems.db
cargo run -q -p lg-content -- check <slug>...     # validate problems without writing the DB
cargo run -q -p lg-content -- judge <slug> f.py   # judge any solution against a problem's suite
cargo run -q -p lg-content -- bench <slug>...     # record reference benchmarks (10 runs)
cargo run -q -p lg-content -- starter <slug> rust # print the starter code for a language
cd app && npx vitest run                          # frontend tests (add --coverage for the gate)
scripts/smoke.sh                                  # end-to-end test of the real desktop app (with timing budgets)
scripts/bump_version.sh major|minor|patch         # cut a SemVer release: CHANGELOG entry, release commit, vX.Y.Z tag
scripts/release.sh [--skip-checks] [--untagged]   # bundled Python + gate (with catalog) + .app/.dmg + smoke (tagged HEAD only)
scripts/fetch_python.sh                           # download the pinned relocatable CPython for bundling
cd app && npx tauri dev                           # run the desktop app (needs assets/problems.db)
cd app && npm run dev:mock                        # UI in a browser against the mock backend (:1421)
swiftc -O scripts/webshot.swift -o /tmp/webshot && /tmp/webshot http://localhost:1421/#/stats out.png
```

## Conventions

- Every change ships with tests; coverage stays above 90% for Rust and the frontend.
- Business logic goes in `localgrind-core`; Tauri commands stay one-line forwards to `Service`.
- UI logic goes in `app/src/lib/*.ts` (unit-tested); components stay thin.
- The mock backend (`app/src/mock/backend.ts`) mirrors command names and payload shapes; update
  it alongside any command change.
- Problem statements and editorials are original writing (see `docs/CONTENT.md`).
- The project follows SemVer (see `README.md`). Record user-visible changes under `[Unreleased]`
  in `CHANGELOG.md` as part of the change; the version lives only in the workspace `Cargo.toml`
  and is bumped by `scripts/bump_version.sh`, never by hand.
