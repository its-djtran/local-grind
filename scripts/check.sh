#!/usr/bin/env bash
# Full project gate: formatting, lints, Rust + frontend tests, the 90% coverage floors, and the
# catalog build with every reference solution judged.
# Usage: scripts/check.sh [--no-coverage]   (--no-coverage: same steps, faster, no coverage floors)
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

coverage=1
[[ "${1:-}" == "--no-coverage" ]] && coverage=0

if ((coverage)); then
  step "rust tests + coverage (floor 90%)"
  log=$(mktemp)
  if ! cargo tarpaulin --engine llvm --workspace --skip-clean --fail-under 90 \
    --exclude-files 'app/src-tauri/src/main.rs' --exclude-files 'app/src-tauri/build.rs' --out Stdout >"$log" 2>&1; then
    grep -E '^\|\| |coverage|FAILED|panicked' "$log" | tail -40
    echo "rust tests or coverage gate failed (full log: $log)"
    exit 1
  fi
  grep -E 'coverage, ' "$log"
else
  step "rust tests"
  cargo test --workspace
fi

step "catalog: every reference solution accepted"
cargo run -q --release -p lg-content -- build
cargo test -q -p localgrind-core --test catalog_e2e -- --ignored

if ((coverage)); then
  step "frontend tests + coverage (floor 90%)"
  (cd app && npx vitest run --coverage)
else
  step "frontend tests"
  (cd app && npx vitest run)
fi

step "all checks passed"
