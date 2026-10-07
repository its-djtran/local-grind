#!/usr/bin/env bash
# End-to-end smoke test of the real desktop app: real WKWebView UI, IPC, Rust backend and
# Python judge. Builds the app, launches it in smoke mode with throwaway user data, and fails
# unless the UI's self-test passes.
set -euo pipefail
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"

[[ -f assets/problems.db ]] || cargo run -q --release -p lg-content -- build
(cd app && npx vite build --logLevel error)
cargo build -q -p localgrind --features custom-protocol

data=$(mktemp -d)
trap 'rm -rf "$data"' EXIT
out=$(LOCALGRIND_SMOKE=1 LOCALGRIND_DATA_DIR="$data" timeout 180 target/debug/localgrind 2>/dev/null) || status=$?
echo "$out" | grep -E '^(ok|FAIL|SMOKE)' || true
[[ "$out" == *"SMOKE PASS"* ]]
