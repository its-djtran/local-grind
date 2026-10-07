#!/usr/bin/env bash
# Full project gate: formatting, lints, Rust + frontend tests, and the 90% coverage floor.
# Usage: scripts/check.sh [--no-coverage]
set -euo pipefail
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"

step() { printf '\n\033[1;33m==> %s\033[0m\n' "$*"; }

step "rustfmt"
cargo fmt --all -- --check

step "clippy"
cargo clippy --workspace --all-targets -- -D warnings

step "frontend type check"
(cd app && npx svelte-check --tsconfig ./tsconfig.json --fail-on-warnings)

if [[ "${1:-}" == "--no-coverage" ]]; then
  step "rust tests"
  cargo test --workspace
  step "frontend tests"
  (cd app && npx vitest run)
  exit 0
fi

step "rust tests + coverage (floor 90%)"
log=$(mktemp)
if ! cargo tarpaulin --engine llvm --workspace --skip-clean --fail-under 90 \
  --exclude-files 'app/src-tauri/src/main.rs' --exclude-files 'app/src-tauri/build.rs' --out Stdout >"$log" 2>&1; then
  grep -E '^\|\| |coverage|FAILED|panicked' "$log" | tail -40
  echo "rust tests or coverage gate failed (full log: $log)"
  exit 1
fi
grep -E 'coverage, ' "$log"

step "frontend tests + coverage (floor 90%)"
(cd app && npx vitest run --coverage)

step "all checks passed"
