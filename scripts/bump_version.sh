#!/usr/bin/env bash
# Cuts a release following SemVer: bumps the version in the workspace Cargo.toml (the single
# source of truth; the app, lg-content and the bundle all read it), turns CHANGELOG.md's
# [Unreleased] section into the new version's entry, and creates the release commit and an
# annotated tag vX.Y.Z. Nothing is pushed.
# Usage: scripts/bump_version.sh major|minor|patch|X.Y.Z
set -euo pipefail
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"
die() {
  echo "error: $*" >&2
  exit 1
}

[[ $# -eq 1 ]] || die "usage: scripts/bump_version.sh major|minor|patch|X.Y.Z"
[[ -z "$(git status --porcelain)" ]] || die "the working tree has uncommitted changes"

current=$(sed -n '/^\[workspace\.package\]/,/^\[/s/^version = "\(.*\)"$/\1/p' Cargo.toml)
semver='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'
[[ $current =~ $semver ]] || die "current version '$current' is not MAJOR.MINOR.PATCH"
major=${BASH_REMATCH[1]} minor=${BASH_REMATCH[2]} patch=${BASH_REMATCH[3]}

case "$1" in
  major) next="$((major + 1)).0.0" ;;
  minor) next="$major.$((minor + 1)).0" ;;
  patch) next="$major.$minor.$((patch + 1))" ;;
  *)
    [[ $1 =~ $semver ]] || die "'$1' is not major, minor, patch or MAJOR.MINOR.PATCH"
    next=$1
    ;;
esac
[[ "$next" != "$current" && "$(printf '%s\n%s\n' "$current" "$next" | sort -V | tail -1)" == "$next" ]] ||
  die "$next is not greater than the current version $current"
! git rev-parse -q --verify "refs/tags/v$next" >/dev/null || die "tag v$next already exists"

grep -q '^## \[Unreleased\]$' CHANGELOG.md || die "CHANGELOG.md has no '## [Unreleased]' section"
awk '/^## \[Unreleased\]$/ { inside = 1; next } /^## \[/ { inside = 0 } inside && NF' CHANGELOG.md | grep -q . ||
  die "CHANGELOG.md's [Unreleased] section is empty: describe the release first"

perl -0pi -e "s/(\[workspace\.package\]\nversion = \")[^\"]+\"/\${1}$next\"/" Cargo.toml
# Records the workspace crates' new version in Cargo.lock without touching dependencies.
cargo metadata --offline --format-version 1 >/dev/null
perl -pi -e "s/^## \[Unreleased\]\$/## [Unreleased]\n\n## [$next] - $(date +%Y-%m-%d)/" CHANGELOG.md

git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -q -m "Release v$next"
git tag -a "v$next" -m "Local Grind $next"
echo "Released v$next (from $current). Publish with: git push origin HEAD v$next"
