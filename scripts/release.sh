#!/usr/bin/env bash
# Builds the distributable macOS app and .dmg:
#   1. fetch the bundled CPython, 2. run the full gate (which builds the problem catalog;
#   with --skip-checks only the catalog is built), 3. tauri build (.app + .dmg) with the catalog
#   and Python as resources,
#   4. smoke-test the built app bundle itself.
# Releases are built from a clean tree whose HEAD carries the tag vX.Y.Z matching the version in
# Cargo.toml (see scripts/bump_version.sh); --untagged allows a local test build of anything else.
# Usage: scripts/release.sh [--skip-checks] [--untagged]
set -euo pipefail
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"
skip_checks=0 untagged=0
for arg in "$@"; do
  case "$arg" in
    --skip-checks) skip_checks=1 ;;
    --untagged) untagged=1 ;;
    *) echo "usage: scripts/release.sh [--skip-checks] [--untagged]" >&2 && exit 2 ;;
  esac
done
version=$(sed -n '/^\[workspace\.package\]/,/^\[/s/^version = "\(.*\)"$/\1/p' Cargo.toml)
if ((!untagged)); then
  [[ -z "$(git status --porcelain)" ]] ||
    { echo "error: uncommitted changes; commit them, or pass --untagged for a local build" >&2 && exit 1; }
  git tag --points-at HEAD | grep -qx "v$version" ||
    { echo "error: HEAD is not tagged v$version; cut a release with scripts/bump_version.sh, or pass --untagged" >&2 && exit 1; }
fi
# macOS has no coreutils `timeout`; perl ships with the OS.
with_timeout() { perl -e 'alarm shift; exec @ARGV' "$@"; }
step() { printf '\n\033[1;33m==> %s\033[0m\n' "$*"; }

step "bundled Python"
scripts/fetch_python.sh

if ((skip_checks)); then
  step "problem catalog"
  cargo run -q --release -p lg-content -- build
else
  step "checks (including the catalog build)"
  scripts/check.sh
fi

step "tauri build"
RESOURCES='{"bundle":{"resources":{"../../assets/problems.db":"problems.db","../../LICENSE-MIT":"LICENSE-MIT","../../LICENSE-APACHE":"LICENSE-APACHE","../../THIRD-PARTY-NOTICES.md":"THIRD-PARTY-NOTICES.md","resources/python/":"python/"}}}'
(cd app && npx tauri build --bundles app,dmg --config "$RESOURCES")

BUNDLE=target/release/bundle
APP=$(ls -d "$BUNDLE"/macos/*.app | head -1)
DMG=$(ls "$BUNDLE"/dmg/*.dmg | head -1)

step "smoke test of the built app"
data=$(mktemp -d)
trap 'rm -rf "$data"' EXIT
out=$(LOCALGRIND_SMOKE=1 LOCALGRIND_DATA_DIR="$data" with_timeout 180 "$APP/Contents/MacOS/localgrind" 2>/dev/null) || true
echo "$out" | grep -E '^(ok|FAIL|SMOKE)'
[[ "$out" == *"SMOKE PASS"* ]]

step "artifacts"
du -sh "$APP" "$DMG"
